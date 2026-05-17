# Geist Design System

> Replica del design system **Geist** di Vercel (https://vercel.com/geist).
> Pensato per essere usato come `DESIGN.md` / contesto per AI coding agents (Claude, Cursor, ecc.) e come riferimento per implementazione manuale.
>
> Fonti primarie: documentazione ufficiale Geist (Foundations: Colors, Typography, Materials, Grid), repo `vercel-labs/skill-remotion-geist` (tokens ufficiali `--ds-*`), package `geist-colors`.

---

## 1. Filosofia & Principi

Geist è un design system ad alto contrasto, ottimizzato per developer tools e prodotti tecnici. La sua identità visiva si poggia su tre pilastri:

1. **Precisione monocromatica** — Il sistema è guidato da una scala di grigi a 10 step. Il colore è funzionale, mai decorativo. L'intera UI può funzionare in solo bianco/nero/grigio; gli accenti (blue, red, amber, green) appaiono solo quando portano informazione (link, errore, warning, success).
2. **Geometria stretta e tipografia pulita** — Tracking negativo sui titoli, font Geist Sans/Mono, gerarchia portata dalla taglia e dal peso, non da decorazioni.
3. **Densità ed efficienza** — Layout densi, dashboard-style. Bordi sottili (1px), shadow ridotte, radius contenuti (6–16px). Niente gradienti su componenti core, niente illustrazioni: solo screenshot reali, code blocks e geometria.

**Anti-patterns:**

- Gradienti su bottoni o card (riservati a hero marketing).
- Shadow pesanti / drop shadows colorate.
- Decorazioni puramente estetiche.
- Letter-spacing positivo sui titoli grandi.
- Border colorate "brand" (l'accento blue è solo per stati attivi e link).

---

## 2. Foundations — Colors

Geist ha **10 scale** di colore, ciascuna con **10 step** (100→1000). I valori `P3` sono usati su browser/display compatibili; gli `hex` riportati sono fallback sRGB.

### 2.1 Convenzioni delle 10 step

Per ogni scala (es. `gray`, `blue`, `red`, ecc.) i 10 step hanno **ruoli semantici fissi**:

| Step     | Ruolo semantico                              |
| -------- | -------------------------------------------- |
| **100**  | Page / app background                        |
| **200**  | Secondary background, subtle differentiation |
| **300**  | Default UI component background              |
| **400**  | Hover background                             |
| **500**  | Active background / default border           |
| **600**  | Hover border                                 |
| **700**  | Active border / **solid brand fill**         |
| **800**  | Hover solid fill / high-contrast bg          |
| **900**  | Secondary text & icons                       |
| **1000** | Primary text & icons (highest contrast)      |

> **Regola d'oro:** se sai a quale "step semantico" serve un colore, puoi sostituire qualsiasi scala (gray → blue → red…) e il layout continuerà a funzionare. Questo è il cuore di Geist.

Le step sono raggruppate in **5 famiglie funzionali**:

1. **Backgrounds** (scale `background`): solo 2 step — `100` (default), `200` (elevated/secondary).
2. **Component backgrounds** (step **1–3** della scala colore = 300, 400, 500 della scala completa nei docs). Default / hover / active.
3. **Borders** (step **4–6** = 500, 600, 700). Default / hover / active.
4. **High-contrast backgrounds** (step **7–8** = 800, 900). Per CTA solidi e badge enfatici.
5. **Text & Icons** (step **9–10** = 900, 1000). Secondary / primary.

> **Nota di nomenclatura:** la documentazione Vercel parla a volte di "Color 1–10" (riferendosi a un sotto-range ravvicinato per UI components); le CSS variables ufficiali (`--ds-gray-100`, `--ds-gray-200`, …, `--ds-gray-1000`) usano la scala 100→1000. Useremo questa seconda convenzione, più granulare.

---

### 2.2 Backgrounds (scala dedicata)

| Token            | CSS Variable          | Light     | Dark      | Uso                              |
| ---------------- | --------------------- | --------- | --------- | -------------------------------- |
| `background-100` | `--ds-background-100` | `#FFFFFF` | `#0A0A0A` | Background di pagina (default)   |
| `background-200` | `--ds-background-200` | `#FAFAFA` | `#171717` | Background secondario / elevated |

**Regola:** usa quasi sempre `background-100`. `background-200` è solo per differenziare sottili elevation (es. sidebar, header sticky).

---

### 2.3 Gray (scala "neutra" — colonna portante del sistema)

| Token       | CSS Variable     | Light hex | Dark hex  | Ruolo                                    |
| ----------- | ---------------- | --------- | --------- | ---------------------------------------- |
| `gray-100`  | `--ds-gray-100`  | `#FAFAFA` | `#1A1A1A` | Subtle background                        |
| `gray-200`  | `--ds-gray-200`  | `#EBEBEB` | `#1F1F1F` | Default component bg                     |
| `gray-300`  | `--ds-gray-300`  | `#E0E0E0` | `#292929` | Hover component bg                       |
| `gray-400`  | `--ds-gray-400`  | `#D1D1D1` | `#2E2E2E` | Active component bg / **default border** |
| `gray-500`  | `--ds-gray-500`  | `#ADADAD` | `#454545` | Hover border                             |
| `gray-600`  | `--ds-gray-600`  | `#878787` | `#5C5C5C` | Active border                            |
| `gray-700`  | `--ds-gray-700`  | `#666666` | `#7A7A7A` | High-contrast bg (solid)                 |
| `gray-800`  | `--ds-gray-800`  | `#404040` | `#A3A3A3` | Hover high-contrast bg                   |
| `gray-900`  | `--ds-gray-900`  | `#171717` | `#EDEDED` | Secondary text & icons                   |
| `gray-1000` | `--ds-gray-1000` | `#0A0A0A` | `#FFFFFF` | Primary text & icons                     |

> Nota: le scale Geist sono **invertite** in dark mode — `gray-100` è quasi-bianco in light e quasi-nero in dark; `gray-1000` è il "foreground" più contrastato in entrambi i temi. Questo permette di usare gli stessi token in entrambe le modalità senza condizionali.

### 2.4 Gray Alpha

Stessa progressione del Gray, ma in `rgba` con alpha crescente. Usata quando il colore deve "fondersi" con un layer sottostante (es. overlay, scrim, hover su immagini). Token: `--ds-gray-alpha-100` … `--ds-gray-alpha-1000`.

---

### 2.5 Accent colors — scale a 10 step

Tutte le accent scale seguono lo stesso pattern semantico. Riporto solo gli step più usati (`100`, `200`, `700`, `900`, `1000`); gli step intermedi seguono interpolazione lineare percettiva e sono disponibili nei tokens ufficiali.

#### Blue — `intent: info / primary action / link`

| Token       | CSS Variable     | Light     | Dark      | Uso                                      |
| ----------- | ---------------- | --------- | --------- | ---------------------------------------- |
| `blue-100`  | `--ds-blue-100`  | `#F2F8FF` | `#0F1B2D` | Subtle bg                                |
| `blue-200`  | `--ds-blue-200`  | `#E5F0FF` | `#0E2348` | Component bg                             |
| `blue-700`  | `--ds-blue-700`  | `#0070F3` | `#0070F3` | **Solid brand fill** — primary CTA, link |
| `blue-800`  | `--ds-blue-800`  | `#0064D9` | `#3291FF` | Hover solid                              |
| `blue-900`  | `--ds-blue-900`  | `#0761D1` | `#52A8FF` | Secondary text                           |
| `blue-1000` | `--ds-blue-1000` | `#0F3F8E` | `#EAF6FF` | Primary text                             |

> `#0070F3` è **il** blu di Vercel — l'unico accento veramente brand del sistema.

#### Red — `intent: error / destructive`

| Token      | CSS Variable    | Light     | Dark      | Uso                |
| ---------- | --------------- | --------- | --------- | ------------------ |
| `red-100`  | `--ds-red-100`  | `#FFF0F0` | `#2A1314` | Subtle bg          |
| `red-200`  | `--ds-red-200`  | `#FFE0E0` | `#3D1719` | Component bg       |
| `red-700`  | `--ds-red-700`  | `#E5484D` | `#E5484D` | Error solid        |
| `red-800`  | `--ds-red-800`  | `#D93D42` | `#FF6166` | Hover destructive  |
| `red-900`  | `--ds-red-900`  | `#CD2B31` | `#FF8589` | Error text         |
| `red-1000` | `--ds-red-1000` | `#7E1A1F` | `#FFE5E5` | Primary error text |

#### Amber — `intent: warning`

| Token        | CSS Variable      | Light     | Dark      | Uso                  |
| ------------ | ----------------- | --------- | --------- | -------------------- |
| `amber-100`  | `--ds-amber-100`  | `#FFFBEB` | `#2B1700` | Subtle bg            |
| `amber-200`  | `--ds-amber-200`  | `#FFF3C4` | `#3D2200` | Component bg         |
| `amber-700`  | `--ds-amber-700`  | `#FFB224` | `#FFB224` | Warning solid        |
| `amber-800`  | `--ds-amber-800`  | `#F1A10D` | `#FFCB47` | Hover warning        |
| `amber-900`  | `--ds-amber-900`  | `#AD5700` | `#F1A10D` | Warning text         |
| `amber-1000` | `--ds-amber-1000` | `#4E2009` | `#FFEFD6` | Primary warning text |

#### Green — `intent: success`

| Token        | CSS Variable      | Light     | Dark      | Uso                  |
| ------------ | ----------------- | --------- | --------- | -------------------- |
| `green-100`  | `--ds-green-100`  | `#EFFCEF` | `#0B2014` | Subtle bg            |
| `green-200`  | `--ds-green-200`  | `#D5F2DD` | `#0F2E1B` | Component bg         |
| `green-700`  | `--ds-green-700`  | `#46A758` | `#46A758` | Success solid        |
| `green-800`  | `--ds-green-800`  | `#3D9A4D` | `#62C073` | Hover success        |
| `green-900`  | `--ds-green-900`  | `#297C3B` | `#7DDB8C` | Success text         |
| `green-1000` | `--ds-green-1000` | `#0E3B1D` | `#E5F8E8` | Primary success text |

#### Teal

| Token       | CSS Variable     | Light     | Dark      | Uso          |
| ----------- | ---------------- | --------- | --------- | ------------ |
| `teal-100`  | `--ds-teal-100`  | `#E6FCF5` | `#082B26` | Subtle bg    |
| `teal-200`  | `--ds-teal-200`  | `#C7F4E6` | `#0C3D36` | Component bg |
| `teal-700`  | `--ds-teal-700`  | `#12A594` | `#12A594` | Solid        |
| `teal-900`  | `--ds-teal-900`  | `#067A6F` | `#5EEAD4` | Text         |
| `teal-1000` | `--ds-teal-1000` | `#053B36` | `#E6FCF5` | Primary text |

#### Purple

| Token         | CSS Variable       | Light     | Dark      | Uso          |
| ------------- | ------------------ | --------- | --------- | ------------ |
| `purple-100`  | `--ds-purple-100`  | `#FBF6FF` | `#1A0F2A` | Subtle bg    |
| `purple-200`  | `--ds-purple-200`  | `#F1E5FF` | `#2A1747` | Component bg |
| `purple-700`  | `--ds-purple-700`  | `#8E4EC6` | `#8E4EC6` | Solid        |
| `purple-900`  | `--ds-purple-900`  | `#793AAF` | `#C99FFF` | Text         |
| `purple-1000` | `--ds-purple-1000` | `#3A1A5A` | `#F2E5FF` | Primary text |

#### Pink

| Token       | CSS Variable     | Light     | Dark      | Uso          |
| ----------- | ---------------- | --------- | --------- | ------------ |
| `pink-100`  | `--ds-pink-100`  | `#FFF0FA` | `#2A0F1F` | Subtle bg    |
| `pink-200`  | `--ds-pink-200`  | `#FCE5F0` | `#451231` | Component bg |
| `pink-700`  | `--ds-pink-700`  | `#F2056F` | `#F2056F` | Solid        |
| `pink-900`  | `--ds-pink-900`  | `#C41C68` | `#FF73B7` | Text         |
| `pink-1000` | `--ds-pink-1000` | `#5A0E33` | `#FFE5F0` | Primary text |

---

### 2.6 Mappatura semantica (consigliata)

Per usare il sistema in modo coerente, mappa i token "fisici" su token "semantici":

```css
:root {
  /* Surfaces */
  --color-bg: var(--ds-background-100);
  --color-bg-subtle: var(--ds-background-200);
  --color-surface: var(--ds-gray-100);
  --color-surface-hover: var(--ds-gray-200);

  /* Borders */
  --color-border: var(--ds-gray-400);
  --color-border-hover: var(--ds-gray-500);
  --color-border-strong: var(--ds-gray-600);

  /* Text */
  --color-fg: var(--ds-gray-1000);
  --color-fg-muted: var(--ds-gray-900);
  --color-fg-subtle: var(--ds-gray-700);

  /* Accent (link, primary) */
  --color-accent: var(--ds-blue-700);
  --color-accent-hover: var(--ds-blue-800);

  /* Status */
  --color-success: var(--ds-green-700);
  --color-warning: var(--ds-amber-700);
  --color-error: var(--ds-red-700);
}
```

---

## 3. Foundations — Typography

Geist usa due font:

- **Geist Sans** — sans-serif geometrico, variable font, weights 100–900. Usato per UI, headings, body, label.
- **Geist Mono** — monospace, variable font, weights 100–900. Usato per codice inline/blocchi, valori tabulari, ID, timestamp, comandi terminale.

```css
:root {
  --font-sans:
    "Geist", ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont,
    "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --font-mono:
    "Geist Mono", ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
    "Liberation Mono", monospace;
}
```

Le classi tipografiche di Geist pre-impostano `font-size`, `line-height`, `letter-spacing` e `font-weight` come una bundle. Sono divise in **4 famiglie**:

### 3.1 Headings — `text-heading-{size}`

Per titoli di pagina/sezione. **Tracking negativo**, semibold (600).

| Classe            | Font-size | Line-height | Letter-spacing | Weight | Uso                                  |
| ----------------- | --------- | ----------- | -------------- | ------ | ------------------------------------ |
| `text-heading-72` | 72px      | 76px        | -0.05em        | 600    | Marketing hero, display              |
| `text-heading-64` | 64px      | 68px        | -0.045em       | 600    | Marketing hero                       |
| `text-heading-56` | 56px      | 60px        | -0.04em        | 600    | Marketing hero                       |
| `text-heading-48` | 48px      | 52px        | -0.04em        | 600    | Marketing                            |
| `text-heading-40` | 40px      | 44px        | -0.035em       | 600    | Marketing                            |
| `text-heading-32` | 32px      | 36px        | -0.03em        | 600    | Marketing subhead, dashboard heading |
| `text-heading-24` | 24px      | 28px        | -0.025em       | 600    | Section title                        |
| `text-heading-20` | 20px      | 24px        | -0.02em        | 600    | Card title                           |
| `text-heading-16` | 16px      | 20px        | -0.015em       | 600    | Subsection title                     |
| `text-heading-14` | 14px      | 18px        | -0.01em        | 600    | Small heading                        |

> Modifier `<strong>` annidato → applica una variante "Subtle" (riduce di un livello l'enfasi, utile per gerarchia mista).

### 3.2 Buttons — `text-button-{size}`

Solo per il testo dentro componenti `<Button>`.

| Classe           | Font-size | Line-height | Weight | Uso                               |
| ---------------- | --------- | ----------- | ------ | --------------------------------- |
| `text-button-16` | 16px      | 20px        | 500    | Largest button                    |
| `text-button-14` | 14px      | 18px        | 500    | Default                           |
| `text-button-12` | 12px      | 16px        | 500    | Tiny button (es. dentro un input) |

### 3.3 Label — `text-label-{size}`

Per testo single-line con altezza ampia per allineamento con icone. Default weight 400, modifier `<strong>` → 500/600.

| Classe               | Font-size | Line-height | Weight | Uso                                                |
| -------------------- | --------- | ----------- | ------ | -------------------------------------------------- |
| `text-label-20`      | 20px      | 32px        | 400    | Marketing text                                     |
| `text-label-18`      | 18px      | 28px        | 400    | —                                                  |
| `text-label-16`      | 16px      | 24px        | 400    | Titoli per differenziarsi da regular               |
| `text-label-14`      | 14px      | 20px        | 400    | **Lo stile di testo più comune** (menu, default)   |
| `text-label-14-mono` | 14px      | 20px        | 400    | Mono pair con label-14+                            |
| `text-label-13`      | 13px      | 18px        | 400    | Secondary line. Modifier "Tabular" per numeri.     |
| `text-label-13-mono` | 13px      | 18px        | 400    | Mono pair con label-14                             |
| `text-label-12`      | 12px      | 16px        | 400    | Tertiary text (Comments, Show More, CAPS calendar) |
| `text-label-12-mono` | 12px      | 16px        | 400    | Mono pair                                          |

### 3.4 Copy — `text-copy-{size}`

Per testo multi-line (paragrafi). `line-height` più ampia rispetto ai Label.

| Classe              | Font-size | Line-height | Weight | Uso                                     |
| ------------------- | --------- | ----------- | ------ | --------------------------------------- |
| `text-copy-24`      | 24px      | 36px        | 400    | Hero marketing                          |
| `text-copy-20`      | 20px      | 30px        | 400    | Hero marketing                          |
| `text-copy-18`      | 18px      | 28px        | 400    | Marketing, big quotes                   |
| `text-copy-16`      | 16px      | 24px        | 400    | Modal body, dove il testo può respirare |
| `text-copy-14`      | 14px      | 22px        | 400    | **Body più comune**                     |
| `text-copy-13`      | 13px      | 20px        | 400    | Secondary text, viste dense             |
| `text-copy-13-mono` | 13px      | 20px        | 400    | Inline code mention                     |

### 3.5 Pattern di accoppiamento

- **Body principale:** `text-copy-14` con `<strong>` (500/600) per enfasi inline.
- **Form label:** `text-label-13` con `<strong>` per il nome del campo, `text-copy-13` per la helper text.
- **Code inline:** `text-copy-13-mono`.
- **ID, timestamp, valori numerici tabulari:** `text-label-13-mono` con modifier "Tabular".
- **Section heading dashboard:** `text-heading-32` (page) → `text-heading-20` (card title).

---

## 4. Foundations — Materials (radii, shadows, elevation)

Geist organizza l'elevation in due gruppi: **Surface** (sulla pagina) e **Floating** (sopra la pagina).

### 4.1 Surface materials

| Classe            | Border       | Background        | Radius | Shadow | Uso                           |
| ----------------- | ------------ | ----------------- | ------ | ------ | ----------------------------- |
| `material-base`   | 1px gray-400 | gray-100          | 6px    | none   | Default container             |
| `material-small`  | 1px gray-400 | gray-100          | 6px    | xs     | Slightly raised (hover)       |
| `material-medium` | 1px gray-400 | bg-100 / gray-100 | 12px   | sm     | Card, panel                   |
| `material-large`  | 1px gray-400 | bg-100            | 12px   | md     | Dialog inline, dashboard hero |

### 4.2 Floating materials

| Classe                | Radius | Shadow | Note                                            |
| --------------------- | ------ | ------ | ----------------------------------------------- |
| `material-tooltip`    | 6px    | xs     | **Solo elemento floating con stem triangolare** |
| `material-menu`       | 12px   | md     | Dropdown, select, command menu                  |
| `material-modal`      | 12px   | lg     | Modal, dialog                                   |
| `material-fullscreen` | 16px   | xl     | Drawer fullscreen, sheet                        |

### 4.3 Radii scale

```css
:root {
  --radius-xs: 4px; /* badge, status dot ring */
  --radius-sm: 6px; /* button, input, surface base/small, tooltip */
  --radius-md: 8px; /* avatar small */
  --radius-lg: 12px; /* card, modal, menu, surface medium/large */
  --radius-xl: 16px; /* fullscreen sheet, drawer */
  --radius-full: 9999px; /* pill, avatar, status dot */
}
```

### 4.4 Shadow scale

Le shadow di Geist sono **strette e a corto raggio**. Niente colore, niente alpha aggressivo.

```css
:root {
  --shadow-xs: 0 1px 2px rgba(0, 0, 0, 0.04);
  --shadow-sm: 0 2px 4px rgba(0, 0, 0, 0.06);
  --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04);
  --shadow-lg: 0 8px 16px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06);
  --shadow-xl: 0 16px 32px rgba(0, 0, 0, 0.12), 0 4px 8px rgba(0, 0, 0, 0.06);
}

/* In dark mode usare alpha più alto + spread minimo per restare leggibili */
[data-theme="dark"] {
  --shadow-xs: 0 1px 2px rgba(0, 0, 0, 0.4);
  --shadow-sm: 0 2px 4px rgba(0, 0, 0, 0.5);
  --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.55), 0 1px 2px rgba(0, 0, 0, 0.4);
  --shadow-lg: 0 8px 16px rgba(0, 0, 0, 0.6), 0 2px 4px rgba(0, 0, 0, 0.5);
  --shadow-xl: 0 16px 32px rgba(0, 0, 0, 0.65), 0 4px 8px rgba(0, 0, 0, 0.5);
}
```

---

## 5. Foundations — Spacing & Grid

Geist usa una **base 4px**. Tutti gli spacing sono multipli di 4.

### 5.1 Spacing scale

| Token       | Value  | px equiv | Uso                           |
| ----------- | ------ | -------- | ----------------------------- |
| `space-0`   | `0`    | 0        | reset                         |
| `space-0_5` | `2px`  | 2        | hairline                      |
| `space-1`   | `4px`  | 4        | gap interno icon+label        |
| `space-2`   | `8px`  | 8        | padding compatto, gap default |
| `space-3`   | `12px` | 12       | input padding-y               |
| `space-4`   | `16px` | 16       | padding card, gap medio       |
| `space-5`   | `20px` | 20       | —                             |
| `space-6`   | `24px` | 24       | section gap                   |
| `space-8`   | `32px` | 32       | block gap                     |
| `space-10`  | `40px` | 40       | large block gap               |
| `space-12`  | `48px` | 48       | section padding               |
| `space-16`  | `64px` | 64       | hero padding                  |
| `space-24`  | `96px` | 96       | marketing section vertical    |

### 5.2 Container widths

| Token           | Max-width | Uso                       |
| --------------- | --------- | ------------------------- |
| `container-sm`  | 640px     | Article, modal narrow     |
| `container-md`  | 768px     | Documentation             |
| `container-lg`  | 1024px    | Dashboard standard        |
| `container-xl`  | 1280px    | Marketing                 |
| `container-2xl` | 1536px    | Marketing wide / full app |

### 5.3 Grid

Geist promuove la **dot grid** (sfondo decorativo a punti) come "huge part of the new Vercel aesthetic" — usata in marketing e in viste vuote.

```css
.dot-grid {
  background-image: radial-gradient(
    circle,
    var(--ds-gray-300) 1px,
    transparent 1px
  );
  background-size: 24px 24px;
}
```

### 5.4 Breakpoints

| Token | Min-width | Device           |
| ----- | --------- | ---------------- |
| `sm`  | 640px     | mobile landscape |
| `md`  | 768px     | tablet           |
| `lg`  | 1024px    | laptop           |
| `xl`  | 1280px    | desktop          |
| `2xl` | 1536px    | wide             |

Mobile-first. Tocco target minimo 36px (preferito 40–44px).

---

## 6. Componenti — Specifiche

> Per ogni componente sono indicati: dimensioni, tokens usati, stati. Geist ha una libreria molto ampia (50+ componenti); riporto i pattern più usati e quelli che definiscono l'identità.

### 6.1 Button

Quattro varianti × tre size.

**Varianti:**

- `primary` (default): bg `gray-1000`, text `gray-100` (= invertito → in light: black bg + white text; in dark: white bg + black text). Hover: `gray-900`.
- `secondary`: bg `background-100`, border `gray-400`, text `gray-1000`. Hover: bg `gray-100`.
- `tertiary` / `ghost`: bg trasparente, no border, text `gray-1000`. Hover: bg `gray-100`.
- `destructive`: bg `red-700`, text `white`. Hover: `red-800`.

**Size:**
| Size | Height | Padding-x | Font class | Radius |
|--------|--------|-----------|-----------------|--------|
| `sm` | 28px | 10px | `text-button-12`| 6px |
| `md` | 36px | 14px | `text-button-14`| 6px |
| `lg` | 44px | 18px | `text-button-16`| 6px |

Stati: `:hover`, `:active`, `:focus-visible` (ring 2px `blue-700` + offset 2px), `:disabled` (opacity 0.5, cursor not-allowed), `loading` (spinner inline + label invisibile).

### 6.2 Input / Textarea

- Height: 36px (default), 28px (sm), 44px (lg).
- Padding-x: 12px.
- Background: `background-100`.
- Border: 1px `gray-400`. Focus: 1px `gray-700` + ring 2px `blue-700` (alpha 0.4).
- Radius: 6px.
- Font: `text-copy-14`.
- Placeholder color: `gray-700`.
- Error state: border `red-700`, helper text `red-900`.
- Success state: border `green-700`.

### 6.3 Badge / Pill

- Height: 20px (xs), 24px (sm), 28px (md).
- Padding-x: 8px.
- Radius: `full` (pill) o 4px (badge squared).
- Font: `text-label-12` con `<strong>` (weight 500), spesso CAPS + tabular per numeri.
- **Varianti per colore:** ogni accent scale fornisce `bg = scale-200` + `text = scale-900` + `border = scale-400` (light); inverso in dark.

### 6.4 Card

- Padding: 16px (compact) / 24px (default) / 32px (large).
- Background: `background-100`.
- Border: 1px `gray-400`.
- Radius: 12px (`material-medium`).
- Shadow: `none` di default; `--shadow-sm` su hover se interattiva.

### 6.5 Modal / Dialog

- Width: 400px (sm), 540px (md), 720px (lg).
- Padding: 24px.
- Background: `background-100`.
- Border: 1px `gray-400`.
- Radius: 12px.
- Shadow: `--shadow-lg`.
- Backdrop: `rgba(0, 0, 0, 0.5)` + `backdrop-filter: blur(4px)`.

### 6.6 Tooltip

- Padding: 6px 8px.
- Background: `gray-1000`.
- Text: `gray-100`, `text-label-12`.
- Radius: 6px.
- Shadow: `--shadow-sm`.
- Arrow: 6px triangolo (unico floating Geist con "stem").

### 6.7 Tabs

- Height: 36px.
- Border-bottom: 1px `gray-400` (sull'intera tab list).
- Active: text `gray-1000`, indicator 2px `gray-1000` border-bottom (sostituisce 1px del wrapper).
- Inactive: text `gray-900`, hover → `gray-1000`.

### 6.8 Avatar

- Size: 20px, 24px, 32px (default), 40px, 48px, 64px.
- Radius: `full`.
- Border: 1px `gray-400` (subtle, sempre presente).
- Fallback: iniziali su bg `gray-200`, text `gray-1000`.

### 6.9 Code Block

- Background: `background-200` (light) / `gray-100` (dark).
- Border: 1px `gray-400`.
- Radius: 8px.
- Font: `Geist Mono`, `text-copy-13-mono` (inline) / `text-copy-14` con font-family override (block).
- Padding: 16px (block), 2px 6px (inline).
- Inline bg: `gray-200`, radius 4px.

### 6.10 Status Dot

Pallino circolare 6–10px che indica stato.

- `success`: `green-700`.
- `warning`: `amber-700`.
- `error`: `red-700`.
- `idle`: `gray-700`.
- Spesso con animazione `ping` (success live).

### 6.11 Note / Callout

Container con border-left colorata 2–4px e bg sottile.

- `info`: `blue-100` bg, `blue-700` border, `blue-1000` text.
- `warning`: `amber-100` bg, `amber-700` border, `amber-1000` text.
- `error`: `red-100` bg, `red-700` border, `red-1000` text.
- `success`: `green-100` bg, `green-700` border, `green-1000` text.

### 6.12 Keyboard Input (Kbd)

Render di tasti tastiera (es. `⌘K`).

- Background: `background-100`.
- Border: 1px `gray-400` + border-bottom 2px `gray-400` (effetto rilievo).
- Radius: 4px.
- Padding: 2px 6px.
- Font: `Geist Mono`, 12px, weight 500.

---

## 7. Motion

Geist privilegia animazioni **brevi**, **damped**, **fisicamente plausibili**. Niente bounce esagerati, niente curve `ease-in-out` lunghe.

```css
:root {
  --duration-instant: 80ms;
  --duration-fast: 150ms;
  --duration-normal: 250ms;
  --duration-slow: 400ms;

  --easing-standard: cubic-bezier(0.4, 0, 0.2, 1);
  --easing-out: cubic-bezier(0, 0, 0.2, 1);
  --easing-in: cubic-bezier(0.4, 0, 1, 1);

  /* Spring (per scale, slide, opacity in eventi UI) */
  /* Equivalente JS: spring({ damping: 200, mass: 1, stiffness: 100 }) */
}
```

**Pattern:**

- Hover bg color → `--duration-fast` linear.
- Modal/drawer enter → `--duration-normal` `--easing-out` + opacity + translate Y.
- Tooltip → `--duration-instant`, fade.
- Button press → scale 0.98 instant, return spring.

---

## 8. Accessibility

- **Contrast:** tutte le combinazioni text/bg semantiche garantiscono WCAG AA (4.5:1) sul body, AAA (7:1) sui titoli grandi. `gray-900` su `background-100` è il limite minimo per body text.
- **Focus visible:** ring 2px `blue-700` con offset 2px su tutti gli elementi interattivi, sempre. Non rimuovere mai.
- **Touch target:** minimo 36×36px (preferito 40+).
- **Motion:** rispettare `prefers-reduced-motion` → durate a 0 e niente spring.
- **Color is not the only signal:** stati di errore/successo affiancano sempre un'icona o testo, mai solo colore.

---

## 9. Dark mode

Geist supporta light/dark con la stessa nomenclatura — i token sono auto-invertiti dalla scala. Implementazione consigliata:

```css
:root {
  color-scheme: light; /* token light */
}
[data-theme="dark"] {
  color-scheme: dark; /* override token dark */
}

@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    color-scheme: dark; /* token dark */
  }
}
```

Uno **Theme Switcher** ufficiale del sistema offre 3 opzioni: `system` / `light` / `dark`.

---

## 10. Layout patterns ricorrenti

### 10.1 Dashboard

- Sidebar fissa 240–280px, bg `background-100`, border-right 1px `gray-400`.
- Top header 56px, bg `background-100`, border-bottom 1px `gray-400`.
- Content area bg `background-100`, padding 24px.
- Card grid: gap 16px, card `material-medium`.

### 10.2 Marketing hero

- Padding vertical 96–128px.
- Heading `text-heading-72` (desktop) → `text-heading-48` (mobile).
- Subhead `text-copy-20`, color `gray-900`.
- CTA primary + ghost.
- Spesso dot-grid background o gradient radiale **scuro** (mai colorato).

### 10.3 Documentation

- Sidebar nav 240px (sticky).
- Content max-width 768px (`container-md`).
- Heading `text-heading-32` per H1 di pagina, `text-heading-24` per H2.
- Body `text-copy-16` line-height 24px.
- TOC right-side, `text-label-13` color `gray-900`.

---

## 11. Do's & Don'ts

**Do:**

- Usare `gray-1000` come foreground primario, sempre.
- Riservare il blue (`blue-700`) a link e primary CTA. Niente decorazione.
- Mantenere border 1px `gray-400` ovunque ci sia un confine.
- Tracking negativo proporzionale alla taglia sui titoli.
- Usare `Geist Mono` per qualunque dato tabulare, ID, timestamp, code.
- Mantenere 1 sola enfasi visiva per gerarchia (size O weight O color, raramente due insieme).

**Don't:**

- Non usare gradienti su componenti UI (button, card). Solo in marketing decorations.
- Non usare ombre colorate o pesanti.
- Non mescolare due accent colors nella stessa view senza ragione semantica (es. blue + purple = no; blue per link + red per error = yes).
- Non aggiungere icone decorative dove non servono. Ogni icona è funzionale.
- Non superare radius 16px tranne pill/avatar.
- Non usare emoji in UI seria (sostituire con icone Geist set).

---

## 12. Quick reference — token table (CSS)

```css
:root {
  /* === Backgrounds === */
  --ds-background-100: #ffffff;
  --ds-background-200: #fafafa;

  /* === Gray scale (light) === */
  --ds-gray-100: #fafafa;
  --ds-gray-200: #ebebeb;
  --ds-gray-300: #e0e0e0;
  --ds-gray-400: #d1d1d1;
  --ds-gray-500: #adadad;
  --ds-gray-600: #878787;
  --ds-gray-700: #666666;
  --ds-gray-800: #404040;
  --ds-gray-900: #171717;
  --ds-gray-1000: #0a0a0a;

  /* === Accent === */
  --ds-blue-700: #0070f3;
  --ds-red-700: #e5484d;
  --ds-amber-700: #ffb224;
  --ds-green-700: #46a758;
  --ds-teal-700: #12a594;
  --ds-purple-700: #8e4ec6;
  --ds-pink-700: #f2056f;

  /* === Typography === */
  --font-sans: "Geist", ui-sans-serif, system-ui, sans-serif;
  --font-mono: "Geist Mono", ui-monospace, monospace;

  /* === Radii === */
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;
  --radius-xl: 16px;
  --radius-full: 9999px;

  /* === Spacing (4px base) === */
  --space-1: 4px;
  --space-2: 8px;
  --space-3: 12px;
  --space-4: 16px;
  --space-6: 24px;
  --space-8: 32px;
  --space-12: 48px;
  --space-16: 64px;
  --space-24: 96px;

  /* === Motion === */
  --duration-fast: 150ms;
  --duration-normal: 250ms;
  --easing-out: cubic-bezier(0, 0, 0.2, 1);
}

[data-theme="dark"] {
  --ds-background-100: #0a0a0a;
  --ds-background-200: #171717;

  --ds-gray-100: #1a1a1a;
  --ds-gray-200: #1f1f1f;
  --ds-gray-300: #292929;
  --ds-gray-400: #2e2e2e;
  --ds-gray-500: #454545;
  --ds-gray-600: #5c5c5c;
  --ds-gray-700: #7a7a7a;
  --ds-gray-800: #a3a3a3;
  --ds-gray-900: #ededed;
  --ds-gray-1000: #ffffff;

  --ds-blue-700: #0070f3;
  --ds-red-700: #e5484d;
  --ds-amber-700: #ffb224;
  --ds-green-700: #46a758;
}
```

---

## 13. Implementation checklist (per replicare il sistema)

1. **Setup font:** installare `geist` package (`npm i geist`) o caricare via CDN jsDelivr (`https://cdn.jsdelivr.net/npm/geist@1.5.1/dist/fonts/`). Esporre come `--font-sans` e `--font-mono`.
2. **CSS custom properties:** copiare il blocco della §12 nel root globale.
3. **Theme switcher:** implementare attributo `data-theme` su `<html>` con `system`/`light`/`dark` (3 opzioni).
4. **Tailwind (consigliato):** mappare le classi `text-heading-*`, `text-copy-*`, `text-label-*`, `text-button-*` come utility custom in `tailwind.config`.
5. **Components base:** Button, Input, Card, Modal, Badge, Tooltip, Tabs, Avatar — partire da questi 8.
6. **Materials:** definire le 4 surface + 4 floating come classi composite (border + radius + shadow + bg).
7. **Icons:** non emoji. Set ufficiale `@geist-ui/icons` o sostituire con `lucide-react` (compatibile, stroke-based, tratto coerente).
8. **Motion utility:** classi `transition-fast` (150ms linear) e `transition-normal` (250ms ease-out).
9. **Focus management:** ring globale `:focus-visible` con `--ds-blue-700`.
10. **QA contrast:** validare ogni combinazione text/bg con un tool tipo Stark / contrast-ratio.

---

## Appendice A — Riferimenti ufficiali

- **Documentazione Geist:** https://vercel.com/geist
- **Foundations:** Colors, Typography, Materials, Grid (sottosezioni della docs)
- **Geist font:** https://vercel.com/font — package `geist` su npm
- **Skill Vercel Labs (tokens ufficiali):** `vercel-labs/skill-remotion-geist` su GitHub
- **Geist colors package community:** `geist-colors` (npm) — mirror dei tokens CSS

> Questo file è un **riferimento di replica** non ufficiale, costruito a partire dalla documentazione pubblica Geist. Vercel mantiene la libreria React effettiva proprietaria; i tokens, le scale e le specifiche sopra sono gli stessi pubblicati nel sito docs.
