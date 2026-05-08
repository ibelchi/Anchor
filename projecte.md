# Pomodoro Flotant — Document de Projecte

> Versió 1.1 | Llicència MIT | Plataforma: Windows | Llengua del codi: Anglès

---

## 1. Visió general

Aplicació Pomodoro per a Windows, lleugera, minimalista i de baixa fricció. Viu com una finestra flotant sempre visible, translúcida i no intrusiva durant l'ús normal. L'usuari interacciona amb ella de forma deliberada i puntual.

### Principis rectors
- **Mínim cognitiu**: l'app no ha de demanar atenció, només oferir-la
- **Baixa fricció**: totes les accions principals en dos passos o menys
- **No intrusiva**: transparent i click-through per defecte
- **Portable**: un sol executable, sense instal·lació

---

## 2. Arquitectura tècnica

### 2.1 Stack tecnològic

| Component | Tecnologia | Justificació |
|---|---|---|
| Llenguatge | **Rust** | Rendiment, binari petit, seguretat de memòria |
| UI framework | **egui** (via eframe) | Lleuger, immediat, sense dependències externes |
| So | **rodio** | Estàndard Rust per àudio, lleuger |
| Finestra | **winit** (via eframe) | Control de finestra natiu, always-on-top, opacitat |
| Configuració | **serde + toml** | Serialització simple, fitxer llegible |
| Build | **cargo** | Estàndard Rust |

### 2.2 Estructura de mòduls

```
src/
├── main.rs              # Punt d'entrada, inicialització eframe
├── app.rs               # Estat global de l'aplicació (App struct)
├── timer.rs             # Lògica del temporitzador, cicle Pomodoro i perfils
├── ui/
│   ├── mod.rs           # Coordinació de la UI
│   ├── timer_view.rs    # Vista principal (números + zona interactuable)
│   ├── context_menu.rs  # Menú contextual (clic dret)
│   └── settings.rs      # Panell de configuració
├── config.rs            # Càrrega/desat de configuració (TOML)
├── audio.rs             # Gestió de sons i notificacions
└── window.rs            # Comportament de finestra (opacitat, click-through, mides)
```

### 2.3 Gestió de l'estat

L'estat de l'aplicació es centralitza en una única estructura `App`:

```rust
// Conceptual — no és codi final
App {
    timer: TimerState,        // fase actual, temps restant, en marxa/parat, comptador cicles
    active_profile: Profile,  // perfil actiu (Clàssic / Sense descans llarg)
    config: Config,           // configuració persistent
    ui_state: UiState,        // menú obert, panell settings visible
    window: WindowState,      // mida actual, mode interacció actiu
}
```

---

## 3. Funcionalitat

### 3.1 Cicle Pomodoro

**Perfil Clàssic:**
```
[TREBALL] → [DESCANS CURT]
[TREBALL] → [DESCANS CURT]
[TREBALL] → [DESCANS CURT]
[TREBALL] → [DESCANS LLARG]  ← cada X cicles (per defecte: 4)
[TREBALL] → [DESCANS CURT]
...
```

**Perfil Sense descans llarg:**
```
[TREBALL] → [DESCANS CURT] → [TREBALL] → [DESCANS CURT] ...
```

Regles comunes a tots dos perfils:
- Durades configurables per perfil
- Canvi de fase automàtic en arribar a zero
- Notificació en cada canvi de fase (so + flash visual)
- El so és diferenciat per a cada tipus de fase (treball, descans curt, descans llarg)
- El comptador de cicles és visible a la UI
- El cicle s'atura en acabar si l'usuari no inicia el següent

### 3.2 Perfils de cicle

Dos perfils fixes inclosos a la v1:

| Perfil | Treball | Descans curt | Descans llarg | Cicles fins descans llarg |
|---|---|---|---|---|
| **Clàssic** | 25 min | 5 min | 15 min | 4 |
| **Sense descans llarg** | 25 min | 5 min | — | — |

- Cada perfil té els seus paràmetres configurables de forma independent
- El canvi de perfil es fa des del menú contextual i reinicia el cicle immediatament
- Perfils múltiples creats per l'usuari: post-v1

### 3.3 Accions principals

| Acció | Com |
|---|---|
| Iniciar / Pausar | Menú contextual → "Iniciar / Pausar" |
| Reiniciar fase | Menú contextual → "Reiniciar" |
| Saltar fase | Menú contextual → "Saltar fase" |
| Canviar perfil | Menú contextual → "Perfil" → [Clàssic / Sense descans llarg] |
| Obrir configuració | Menú contextual → "Configuració" |
| Tancar app | Menú contextual → "Tancar" |

### 3.4 Notificacions

- **So**: fitxer d'àudio breu reproduït via rodio
- **Flash visual**: la finestra puja d'opacitat bruscament i torna al valor normal en ~1 segon
- No es fan servir notificacions toast de Windows (massa intrusives)

---

## 4. Interfície d'usuari

### 4.1 Finestra principal

La finestra no té decoració nativa de Windows (sense barra de títol, sense botons de tancar). Tot el comportament és custom.

**Contingut visible:**
```
┌─────────────┐
│   24:59     │  ← timer, tipografia gran, centrat
│  [treball]  │  ← etiqueta de fase, discreta
│   ● ● ● ○   │  ← comptador de cicles (● completat, ○ pendent)
└─────────────┘
```

El comptador de cicles només és visible en el perfil Clàssic. En el perfil Sense descans llarg s'omet.

La zona del timer és l'**única àrea sempre interactuable**, fins i tot amb click-through actiu. Té una mida mínima garantida (aprox. 60×30 px al centre) que captura el clic dret per obrir el menú.

### 4.2 Mides predefinides

| Mida | Dimensions aprox. | Ús típic |
|---|---|---|
| S | 120 × 60 px | Monitors grans, posició cantonada |
| M | 180 × 90 px | Ús general (per defecte) |
| L | 260 × 130 px | Monitors petits o visió reduïda |

Canvi de mida des del menú de configuració.

### 4.3 Comportament visual per estat

| Estat | Opacitat | Click-through |
|---|---|---|
| Normal (en marxa) | 25–40% | Sí (excepte zona central) |
| Notificació (flash) | 90% → baixa | Sí |
| Mode interacció | 90% | No |

El "mode interacció" s'activa en fer clic a la zona central. Es desactiva automàticament en tancar el menú o després d'un timeout breu (ex: 3 segons sense activitat).

### 4.4 Colors per fase

| Fase | Color accent |
|---|---|
| Treball | Vermell suau (#E05C5C o similar) |
| Descans curt | Verd suau (#5CB85C o similar) |
| Descans llarg | Blau suau (#5C9BE0 o similar) |

Els colors són configurables per perfil.

### 4.5 Arrossegament

La finestra és draggable des de qualsevol punt. Com no té barra de títol, s'implementa drag custom: clic mantingut + moviment en qualsevol zona de la finestra (quan el mode interacció és actiu).

---

## 5. Configuració

### 5.1 Paràmetres configurables

Els paràmetres de cicle es configuren per perfil de forma independent:

| Paràmetre | Tipus | Per defecte (Clàssic) | Per defecte (Sense descans llarg) |
|---|---|---|---|
| Durada treball | minuts | 25 | 25 |
| Durada descans curt | minuts | 5 | 5 |
| Durada descans llarg | minuts | 15 | — |
| Cicles fins descans llarg | enter | 4 | — |
| Color treball | hex | #E05C5C | #E05C5C |
| Color descans curt | hex | #5CB85C | #5CB85C |
| Color descans llarg | hex | #5C9BE0 | — |

Paràmetres globals (independents del perfil):

| Paràmetre | Tipus | Per defecte |
|---|---|---|
| Opacitat base | % | 30 |
| So activat | bool | true |
| Volum so | % | 70 |
| Mida finestra | S/M/L | M |
| Always on top | bool | true |

### 5.2 Persistència

La configuració es desa en un fitxer TOML a la mateixa carpeta de l'executable (app portable):

```
pomodoro.exe
config.toml       ← creat automàticament en primera execució
```

### 5.3 Panell de configuració (UI)

S'obre com una finestra secundària petita quan l'usuari tria "Configuració" al menú contextual. No és modal (no bloqueja). Té un botó "Desa" i un "Cancel·la". Els canvis s'apliquen en temps real on sigui possible (opacitat, mida).

---

## 6. Comportament de finestra

### 6.1 Click-through parcial

La solució tècnica és:

1. La finestra principal té click-through activat globalment via Windows API (`WS_EX_TRANSPARENT`)
2. Una subregió central (la zona del timer) **no** té click-through: s'implementa fent hit-testing custom — quan el clic cau dins la zona central, s'intercepta; fora, es deixa passar

Això es gestiona des de `window.rs` i requereix crides a l'API Win32.

### 6.2 Always on top

Activat per defecte via `HWND_TOPMOST`. Configurable.

### 6.3 Posició inicial

Primera execució: cantonada inferior dreta amb marge de 20px. Les execucions posteriors recorden la posició de l'última sessió (desat a config.toml).

---

## 7. So

- Tres sons inclosos al binari (embed amb `include_bytes!`):
  - Fi de fase de treball
  - Fi de descans curt
  - Fi de descans llarg (so diferenciat, més destacat)
- Format: OGG o WAV breu (< 2 segons)
- Volum i activació configurables globalment
- Si el so falla (dispositiu no disponible), l'app continua sense errors

---

## 8. Fases de desenvolupament

### Fase 1 — Esquelet funcional
- [ ] Projecte Rust amb eframe
- [ ] Finestra sense decoració, always-on-top
- [ ] Timer bàsic que compta enrere
- [ ] Cicle complet: treball / descans curt / descans llarg
- [ ] Comptador de cicles intern
- [ ] Dos perfils: Clàssic i Sense descans llarg

### Fase 2 — Interacció
- [ ] Menú contextual (clic dret)
- [ ] Accions: iniciar, pausar, reiniciar, saltar, tancar
- [ ] Canvi de perfil des del menú contextual
- [x] Drag de finestra

### Fase 3 — Comportament de finestra
- [ ] Opacitat configurable
- [ ] Click-through global + zona interactuable central
- [ ] Mides S/M/L

### Fase 4 — Notificacions
- [x] Integració rodio + tres sons inclosos (treball, descans curt, descans llarg)
- [x] Flash visual en canvi de fase
- [ ] Comptador de cicles visible a la UI (perfil Clàssic)

### Fase 5 — Configuració
- [ ] Estructura Config + serde/toml amb paràmetres per perfil
- [ ] Panell de configuració UI (pestanyes per perfil)
- [ ] Persistència de posició i configuració

### Fase 6 — Poliment
- [ ] Colors per fase (treball, descans curt, descans llarg) configurables per perfil
- [ ] Ajust tipografia i proporcions per mida
- [ ] Test en diverses resolucions i DPI
- [ ] Build release (mida executable, strip symbols)
- [ ] README i llicència MIT

---

## 9. No-objectius

- No hi ha seguiment de sessions ni estadístiques
- No hi ha integració amb calendaris ni tasques
- No hi ha notificacions toast de Windows
- No hi ha actualitzacions automàtiques
- No hi ha perfils de cicle personalitzats per l'usuari (només els dos fixes de v1)
- La configuració manual del TOML és possible però no és el camí principal

---

## 10. Decisions obertes / futures (post-v1)

- Perfils de cicle personalitzats creats per l'usuari
- Hotkey global com a alternativa al clic a la zona central
- So personalitzat (fitxer extern configurable)
- Versió Linux/macOS si egui ho permet sense canvis majors

---

*Fi del document de projecte v1.1*
