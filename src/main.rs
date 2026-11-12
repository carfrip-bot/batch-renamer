mod engine;

use eframe::egui;
use engine::{apply_rules, invalid_name, load_presets, save_presets, split_name, Kind, Preset, Rule};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

struct Entry {
    path: PathBuf,
    checked: bool,
    new_name: String,
    status: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Language {
    Italian,
    English,
}

impl Language {
    fn text(self, english: &'static str) -> &'static str {
        if self == Language::English {
            if english == "Ready plural" {
                return "ready";
            }
            return english;
        }
        match english {
            "Add files..." => "Aggiungi file...",
            "Add folder..." => "Aggiungi cartella...",
            "Include subfolders" => "Includi sottocartelle",
            "Validate" => "Verifica",
            "Rename" => "Rinomina",
            "Undo last rename" => "Annulla ultima rinomina",
            "Mark all" => "Seleziona tutti",
            "Unmark all" => "Deseleziona tutti",
            "Sort by name" => "Ordina per nome",
            "Clear list" => "Svuota elenco",
            "Rules" => "Regole",
            "Add rule" => "Aggiungi regola",
            "Remove all" => "Rimuovi tutte",
            "Up" => "Su",
            "Down" => "Giù",
            "Copy" => "Copia",
            "Delete" => "Elimina",
            "Presets" => "Modelli",
            "Save current rules" => "Salva regole",
            "Preset name" => "Nome del modello",
            "Load" => "Carica",
            "Name" => "Nome",
            "New name" => "Nuovo nome",
            "Status" => "Stato",
            "Folder" => "Cartella",
            "Text:" => "Testo:",
            "Prefix" => "Prefisso",
            "Suffix" => "Suffisso",
            "At position" => "A posizione",
            "Position:" => "Posizione:",
            "Count from the right" => "Conta da destra",
            "By position" => "Per posizione",
            "Between texts" => "Tra due testi",
            "From:" => "Da:",
            "Count:" => "Caratteri:",
            "From the right" => "Da destra",
            "From text:" => "Dal testo:",
            "Until text:" => "Fino al testo:",
            "Keep the two texts" => "Mantieni i due testi",
            "Find:" => "Cerca:",
            "With:" => "Sostituisci con:",
            "First" => "Prima occorrenza",
            "All" => "Tutte",
            "Last" => "Ultima occorrenza",
            "Case sensitive" => "Maiuscole/minuscole",
            "Set to" => "Imposta",
            "Lowercase" => "Minuscolo",
            "Uppercase" => "Maiuscolo",
            "Remove" => "Rimuovi",
            "Add if missing" => "Aggiungi se assente",
            "Extension:" => "Estensione:",
            "Digits" => "Numeri",
            "Digits:" => "Cifre:",
            "Invalid expression" => "Espressione non valida",
            "Letters" => "Lettere",
            "Symbols" => "Simboli",
            "Also these characters:" => "Anche questi caratteri:",
            "lowercase" => "minuscolo",
            "UPPERCASE" => "MAIUSCOLO",
            "Each Word" => "Iniziale di ogni parola",
            "First letter" => "Iniziale maiuscola",
            "Invert" => "Inverti",
            "Before name" => "Prima del nome",
            "After name" => "Dopo il nome",
            "Replace name" => "Sostituisci il nome",
            "Start:" => "Inizio:",
            "Step:" => "Passo:",
            "Separator:" => "Separatore:",
            "Order:" => "Ordine:",
            "Ascending" => "Crescente",
            "Descending" => "Decrescente",
            "Text before the number:" => "Testo prima del numero:",
            "Text after the number:" => "Testo dopo il numero:",
            "Descending: the first file gets the highest number." => {
                "Decrescente: il primo file riceve il numero più alto."
            }
            "Length:" => "Lunghezza:",
            "Seed:" => "Seme:",
            "Trim spaces at both ends" => "Rimuovi spazi iniziali e finali",
            "Underscores to spaces" => "Sostituisci _ con spazi",
            "Collapse repeated spaces" => "Riduci gli spazi ripetuti",
            "Remove text in brackets" => "Rimuovi il testo tra parentesi",
            "Pattern:" => "Espressione:",
            "Replace with:" => "Sostituisci con:",
            "First match" => "Prima corrispondenza",
            "All matches" => "Tutte le corrispondenze",
            "Ignore case" => "Ignora maiuscole/minuscole",
            "Use ${1}, ${2} ... for groups." => "Usa ${1}, ${2}... per i gruppi.",
            "Split at any of:" => "Separa in corrispondenza di:",
            "New pattern:" => "Nuovo schema:",
            "{1} is the first part, {2} the second, {0} the whole name." => {
                "{1} è la prima parte, {2} la seconda, {0} il nome intero."
            }
            "Add zeros" => "Aggiungi zeri",
            "Remove zeros" => "Rimuovi zeri",
            "Width:" => "Larghezza:",
            "Replaces accented letters with plain ones (for example é becomes e)." => {
                "Sostituisce le lettere accentate con quelle semplici (per esempio, é diventa e)."
            }
            "One new name per line; line 1 goes to file 1, and so on." => {
                "Inserisci un nome per riga: la prima riga corrisponde al primo file."
            }
            "One pair per line: old name, then a Tab or =, then new name." => {
                "Una coppia per riga: nome originale, Tab o =, nuovo nome."
            }
            "Skip extension" => "Non modificare l'estensione",
            "Language" => "Lingua",
            "Theme" => "Tema",
            "Light" => "Chiaro",
            "Dark" => "Scuro",
            "Drop files or folders here" => "Trascina qui file o cartelle",
            "or use the buttons above to get started" => {
                "oppure usa i pulsanti in alto per iniziare"
            }
            "Preview" => "Anteprima",
            "No files added" => "Nessun file aggiunto",
            "Ready" => "Pronto",
            "Unchanged" => "Invariato",
            "Skipped (unmarked)" => "Ignorato (non selezionato)",
            "Invalid name" => "Nome non valido",
            "Name too long" => "Nome troppo lungo",
            "Duplicate name" => "Nome duplicato",
            "Already exists" => "Esiste già",
            "Rename failed" => "Rinomina non riuscita",
            "All files" => "Tutti i file",
            "Marked" => "Selezionati",
            "No rules" => "Nessuna regola",
            "Add a rule to build your rename" => "Aggiungi regole per comporre la rinomina",
            "No presets saved" => "Nessun modello salvato",
            "Type a name for this preset" => "Dai un nome a questo modello",
            "Files and folders added" => "File e cartelle aggiunti",
            "Drag files here to add them" => "Trascina qui i file per aggiungerli",
            "Added {} file(s)." => "Aggiunti {} file.",
            "Loaded {} file(s)." => "Caricati {} file.",
            "List cleared." => "Elenco svuotato.",
            "Preset deleted." => "Modello eliminato.",
            "Ready plural" => "pronti",
            "Validated successfully. {} file(s) ready to rename." => {
                "Verifica completata: {} file pronti da rinominare."
            }
            "Problems found in {} file(s). Fix them or unmark those files. {} ready." => {
                "Trovati problemi in {} file. Correggili o deseleziona i file. {} pronti."
            }
            "Nothing to rename." => "Nessun file da rinominare.",
            "Renamed {} file(s)." => "Rinominati {} file.",
            "Rename failed: {}" => "Rinomina non riuscita: {}",
            "Nothing to undo!" => "Nessuna rinomina da annullare.",
            "Undo: {} out of {} file(s) restored." => {
                "Annullamento: ripristinati {} file su {}."
            }
            "Undo failed: {}" => "Annullamento non riuscito: {}",
            "Preset saved." => "Modello salvato.",
            "Could not save presets: {}" => "Impossibile salvare i modelli: {}",
            _ => english,
        }
    }

    fn kind_label(self, kind: Kind) -> &'static str {
        if self == Language::English {
            return kind.label();
        }
        match kind {
            Kind::Insert => "Inserisci",
            Kind::Delete => "Elimina caratteri",
            Kind::Remove => "Rimuovi testo",
            Kind::Replace => "Sostituisci testo",
            Kind::Extension => "Estensione",
            Kind::Strip => "Rimuovi caratteri",
            Kind::Case => "Maiuscole/minuscole",
            Kind::Number => "Serializza",
            Kind::Numbering => "Numerazione",
            Kind::Randomize => "Nome casuale",
            Kind::CleanUp => "Pulisci",
            Kind::Regex => "Espressione regolare",
            Kind::Rearrange => "Riordina",
            Kind::Padding => "Zeri iniziali",
            Kind::Transliterate => "Traslittera",
            Kind::AssignNames => "Nomi da elenco",
            Kind::MapNames => "Mappa nomi",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThemeMode {
    Light,
    Dark,
}

fn apply_theme(ctx: &egui::Context, theme: ThemeMode) {
    let (accent, hover, active, quiet_green, border) = match theme {
        ThemeMode::Light => (
            egui::Color32::from_rgb(35, 132, 83),
            egui::Color32::from_rgb(222, 241, 229),
            egui::Color32::from_rgb(35, 132, 83),
            egui::Color32::from_rgb(239, 247, 242),
            egui::Color32::from_rgb(192, 220, 202),
        ),
        ThemeMode::Dark => (
            egui::Color32::from_rgb(99, 203, 145),
            egui::Color32::from_rgb(48, 76, 60),
            egui::Color32::from_rgb(55, 145, 96),
            egui::Color32::from_rgb(35, 48, 40),
            egui::Color32::from_rgb(69, 100, 80),
        ),
    };
    let mut visuals = match theme {
        ThemeMode::Light => egui::Visuals::light(),
        ThemeMode::Dark => egui::Visuals::dark(),
    };
    let theme_text = visuals.text_color();
    visuals.hyperlink_color = accent;
    let (selection_bg, selection_fg) = match theme {
        ThemeMode::Light => (
            egui::Color32::from_rgb(190, 226, 205),
            egui::Color32::from_rgb(16, 72, 44),
        ),
        ThemeMode::Dark => (
            egui::Color32::from_rgb(50, 105, 75),
            egui::Color32::from_rgb(232, 247, 238),
        ),
    };
    visuals.selection.bg_fill = selection_bg;
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, selection_fg);
    visuals.widgets.inactive.bg_fill = quiet_green;
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, border);
    visuals.widgets.hovered.bg_fill = hover;
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, accent);
    visuals.widgets.active.bg_fill = active;
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, theme_text);
    visuals.widgets.open.bg_fill = hover;
    visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0_f32, accent);
    ctx.set_visuals(visuals);
    ctx.style_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(10.0, 6.0);
        style.spacing.interact_size.y = 28.0;
    });
}

fn primary_text_color(theme: ThemeMode) -> egui::Color32 {
    match theme {
        ThemeMode::Light => egui::Color32::from_rgb(43, 54, 47),
        ThemeMode::Dark => egui::Color32::from_rgb(225, 235, 228),
    }
}

fn localized_status(language: Language, status: &str) -> String {
    match status {
        "Ready" => language.text("Ready").to_string(),
        "Unchanged" => language.text("Unchanged").to_string(),
        "Skipped (unmarked)" => language.text("Skipped (unmarked)").to_string(),
        "Invalid name" => language.text("Invalid name").to_string(),
        "Name too long" => language.text("Name too long").to_string(),
        "Duplicate name" => language.text("Duplicate name").to_string(),
        "Already exists" => language.text("Already exists").to_string(),
        "Rename failed" => language.text("Rename failed").to_string(),
        other => other.to_string(),
    }
}

fn status_color(status: &str, theme: ThemeMode) -> egui::Color32 {
    match status {
        "Ready" => match theme {
            ThemeMode::Light => egui::Color32::from_rgb(30, 126, 76),
            ThemeMode::Dark => egui::Color32::from_rgb(108, 215, 151),
        },
        "Invalid name" | "Name too long" | "Duplicate name" | "Already exists" | "Rename failed" => {
            match theme {
                ThemeMode::Light => egui::Color32::from_rgb(174, 55, 47),
                ThemeMode::Dark => egui::Color32::from_rgb(255, 126, 115),
            }
        }
        _ => match theme {
            ThemeMode::Light => egui::Color32::from_rgb(94, 103, 96),
            ThemeMode::Dark => egui::Color32::from_rgb(177, 186, 179),
        },
    }
}

struct App {
    files: Vec<Entry>,
    rules: Vec<Rule>,
    presets: Vec<Preset>,
    new_kind: Kind,
    preset_name: String,
    message: String,
    undo: Vec<(PathBuf, PathBuf)>,
    subfolders: bool,
    snapshot: String,
    files_dirty: bool,
    screenshot_path: Option<String>,
    frames: u32,
    language: Language,
    theme: ThemeMode,
}

fn path_key(path: &Path) -> String {
    path.to_string_lossy().to_lowercase()
}

fn staging_path(path: &Path, index: usize, reserved: &mut HashSet<String>) -> PathBuf {
    for attempt in 0u64.. {
        let name = format!(
            ".batch-renamer-{}-{}-{}.tmp",
            std::process::id(),
            index,
            attempt
        );
        let candidate = path.with_file_name(name);
        if !candidate.exists() && reserved.insert(path_key(&candidate)) {
            return candidate;
        }
    }
    unreachable!("an unused staging filename should always be available")
}

/// Renames a group in two phases so cycles and chains can be handled safely.
/// On failure, `locations` reports where each source file ended up.
fn rename_batch(pairs: &[(PathBuf, PathBuf)]) -> Result<(), (String, Vec<PathBuf>)> {
    let mut reserved: HashSet<String> = pairs
        .iter()
        .flat_map(|(from, to)| [path_key(from), path_key(to)])
        .collect();
    let staging: Vec<PathBuf> = pairs
        .iter()
        .enumerate()
        .map(|(i, (from, _))| staging_path(from, i, &mut reserved))
        .collect();
    let mut locations: Vec<PathBuf> = pairs.iter().map(|(from, _)| from.clone()).collect();

    for (i, (from, _)) in pairs.iter().enumerate() {
        if let Err(error) = std::fs::rename(from, &staging[i]) {
            let mut rollback_errors = Vec::new();
            for j in (0..i).rev() {
                if let Err(rollback) = std::fs::rename(&staging[j], &pairs[j].0) {
                    rollback_errors.push(format!(
                        "could not restore '{}' from '{}': {}",
                        pairs[j].0.display(),
                        staging[j].display(),
                        rollback
                    ));
                } else {
                    locations[j] = pairs[j].0.clone();
                }
            }
            let detail = if rollback_errors.is_empty() {
                String::new()
            } else {
                format!(" Rollback issue: {}", rollback_errors.join("; "))
            };
            return Err((
                format!("Could not stage '{}': {}.{}", from.display(), error, detail),
                locations,
            ));
        }
        locations[i] = staging[i].clone();
    }

    for (i, (_, to)) in pairs.iter().enumerate() {
        let move_result = if to.exists() {
            Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "destination became occupied",
            ))
        } else {
            std::fs::rename(&staging[i], to)
        };
        if let Err(error) = move_result {
            let mut rollback_errors = Vec::new();
            // Move completed destinations back to their private staging names first.
            // This clears cycles before restoring any original source names.
            for j in (0..i).rev() {
                if let Err(rollback) = std::fs::rename(&pairs[j].1, &staging[j]) {
                    rollback_errors.push(format!(
                        "could not stage '{}' for recovery: {}",
                        pairs[j].1.display(),
                        rollback
                    ));
                } else {
                    locations[j] = staging[j].clone();
                }
            }
            if rollback_errors.is_empty() {
                for j in (0..pairs.len()).rev() {
                    if locations[j] == staging[j] {
                        if let Err(rollback) = std::fs::rename(&staging[j], &pairs[j].0) {
                            rollback_errors.push(format!(
                                "could not restore '{}' from '{}': {}",
                                pairs[j].0.display(),
                                staging[j].display(),
                                rollback
                            ));
                        } else {
                            locations[j] = pairs[j].0.clone();
                        }
                    }
                }
            }
            let detail = if rollback_errors.is_empty() {
                String::new()
            } else {
                format!(" Recovery issue: {}", rollback_errors.join("; "))
            };
            return Err((
                format!("Could not rename to '{}': {}.{}", to.display(), error, detail),
                locations,
            ));
        }
        locations[i] = to.clone();
    }

    Ok(())
}

impl App {
    fn new(screenshot_path: Option<String>, theme: ThemeMode) -> App {
        let mut app = App {
            files: Vec::new(),
            rules: Vec::new(),
            presets: load_presets(),
            new_kind: Kind::Insert,
            preset_name: String::new(),
            message: "Trascina qui i file o usa i pulsanti in alto per iniziare.".to_string(),
            undo: Vec::new(),
            subfolders: false,
            snapshot: String::new(),
            files_dirty: true,
            screenshot_path,
            frames: 0,
            language: Language::Italian,
            theme,
        };
        if app.screenshot_path.is_some() {
            app.load_demo();
            app.message = app
                .language
                .text("Loaded {} file(s).")
                .replace("{}", &app.files.len().to_string());
        }
        app
    }

    fn load_demo(&mut self) {
        let names = [
            "IMG_0412 (copy).JPG",
            "IMG_0413 (copy).JPG",
            "holiday   photo_final.png",
            "Report_2023-final.docx",
            "song - artist.mp3",
            "notes.txt",
        ];
        for n in names.iter() {
            self.files.push(Entry {
                path: PathBuf::from(format!("C:\\Demo\\{}", n)),
                checked: true,
                new_name: String::new(),
                status: String::new(),
            });
        }
        let mut r = Rule::new(Kind::CleanUp);
        r.flag4 = true;
        self.rules.push(r);
        let mut c = Rule::new(Kind::Case);
        c.choice = 0;
        self.rules.push(c);
        let mut n = Rule::new(Kind::Number);
        n.choice = 1;
        self.rules.push(n);
        let mut e = Rule::new(Kind::Extension);
        e.choice = 1;
        e.skip_ext = false;
        self.rules.push(e);
        self.files_dirty = true;
    }

    fn add_paths(&mut self, paths: Vec<PathBuf>) -> usize {
        let before = self.files.len();
        for p in paths {
            if self.files.iter().any(|f| path_key(&f.path) == path_key(&p)) {
                continue;
            }
            self.files.push(Entry {
                path: p,
                checked: true,
                new_name: String::new(),
                status: String::new(),
            });
        }
        self.files_dirty = true;
        self.files.len() - before
    }

    fn set_added_message(&mut self, count: usize) {
        self.message = if count == 0 {
            if self.language == Language::Italian {
                "Nessun nuovo file aggiunto.".to_string()
            } else {
                "No new files added.".to_string()
            }
        } else {
            self.language
                .text("Added {} file(s).")
                .replace("{}", &count.to_string())
        };
    }

    fn add_dropped_paths(&mut self, paths: Vec<PathBuf>) -> usize {
        let mut files = Vec::new();
        let mut folders = Vec::new();
        for path in paths {
            if path.is_dir() {
                folders.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
        let before = self.files.len();
        self.add_paths(files);
        for folder in folders {
            self.add_folder(&folder, 0);
        }
        self.files.len() - before
    }

    fn add_folder(&mut self, dir: &Path, depth: u32) -> usize {
        let mut found: Vec<PathBuf> = Vec::new();
        let mut subs: Vec<PathBuf> = Vec::new();
        if let Ok(rd) = std::fs::read_dir(dir) {
            for item in rd.flatten() {
                let p = item.path();
                match item.file_type() {
                    Ok(ft) if ft.is_dir() => subs.push(p),
                    // linked folders could loop forever: skip them
                    Ok(ft) if ft.is_symlink() && p.is_dir() => {}
                    _ => found.push(p),
                }
            }
        }
        found.sort();
        let mut added = self.add_paths(found);
        if self.subfolders && depth < 32 {
            subs.sort();
            for s in subs {
                added += self.add_folder(&s, depth + 1);
            }
        }
        added
    }

    fn recompute(&mut self) {
        let mut parents_new: HashMap<(String, String), usize> = HashMap::new();
        // Numbering counts only marked files, so unmarking one leaves no gap.
        let mut counter = 0usize;
        let total = self.files.iter().filter(|f| f.checked).count();
        for f in self.files.iter_mut() {
            let name = f.path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            let (stem, ext) = split_name(&name);
            let (ns, ne) = apply_rules(&self.rules, &stem, &ext, counter, total);
            if f.checked {
                counter += 1;
            }
            // An empty stem would produce a hidden ".ext" file: report it as invalid.
            f.new_name = if ns.is_empty() {
                String::new()
            } else if ne.is_empty() {
                ns
            } else {
                format!("{}.{}", ns, ne)
            };
        }
        for f in self.files.iter() {
            if f.checked && !f.new_name.is_empty() {
                let parent = f.path.parent().map(|p| p.to_string_lossy().to_lowercase()).unwrap_or_default();
                *parents_new.entry((parent, f.new_name.to_lowercase())).or_insert(0) += 1;
            }
        }
        for f in self.files.iter_mut() {
            let old = f.path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            let parent = f.path.parent().map(|p| p.to_string_lossy().to_lowercase()).unwrap_or_default();
            f.status = if !f.checked {
                "Skipped (unmarked)".to_string()
            } else if f.new_name == old {
                "Unchanged".to_string()
            } else if invalid_name(&f.new_name) {
                "Invalid name".to_string()
            } else if f.new_name.chars().count() > 255 {
                "Name too long".to_string()
            } else if parents_new.get(&(parent.clone(), f.new_name.to_lowercase())).copied().unwrap_or(0) > 1 {
                "Duplicate name".to_string()
            } else {
                "Ready".to_string()
            };
        }

        // A destination occupied by another file is available only when that
        // file is also in the batch and will actually move. Repeat because a
        // blocked file can in turn block another file targeting its old name.
        let exists: Vec<bool> = self
            .files
            .iter()
            .map(|f| f.status == "Ready" && f.path.with_file_name(&f.new_name).exists())
            .collect();
        loop {
            let moving: HashSet<String> = self
                .files
                .iter()
                .filter(|f| f.status == "Ready")
                .map(|f| path_key(&f.path))
                .collect();
            let mut changed = false;
            for (i, f) in self.files.iter_mut().enumerate() {
                if f.status != "Ready" || !exists[i] {
                    continue;
                }
                let target = f.path.with_file_name(&f.new_name);
                if !moving.contains(&path_key(&target)) {
                    f.status = "Already exists".to_string();
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        self.files_dirty = false;
    }

    /// Recomputes the preview when rules or files changed. Returns true if it did.
    fn refresh(&mut self) -> bool {
        let snap = serde_json::to_string(&self.rules).unwrap_or_default();
        if snap != self.snapshot || self.files_dirty {
            self.snapshot = snap;
            self.recompute();
            true
        } else {
            false
        }
    }

    fn validate(&mut self) {
        self.recompute();
        let bad = self
            .files
            .iter()
            .filter(|f| matches!(f.status.as_str(), "Invalid name" | "Name too long" | "Duplicate name" | "Already exists"))
            .count();
        let ready = self.files.iter().filter(|f| f.status == "Ready").count();
        if bad == 0 {
            self.message = if self.language == Language::Italian {
                format!("Verifica completata: {} file pronti da rinominare.", ready)
            } else {
                format!("Validated successfully. {} file(s) ready to rename.", ready)
            };
        } else {
            self.message = if self.language == Language::Italian {
                format!("Trovati problemi in {} file. Correggili o deselezionali. {} pronti.", bad, ready)
            } else {
                format!("Problems found in {} file(s). Fix them or unmark those files. {} ready.", bad, ready)
            };
        }
    }

    fn rename(&mut self) {
        self.recompute();
        let ready: Vec<(usize, PathBuf, PathBuf)> = self
            .files
            .iter()
            .enumerate()
            .filter(|(_, f)| f.status == "Ready")
            .map(|(i, f)| (i, f.path.clone(), f.path.with_file_name(&f.new_name)))
            .collect();
        if ready.is_empty() {
            self.message = self.language.text("Nothing to rename.").to_string();
            return;
        }
        let pairs: Vec<(PathBuf, PathBuf)> = ready
            .iter()
            .map(|(_, from, to)| (from.clone(), to.clone()))
            .collect();
        match rename_batch(&pairs) {
            Ok(()) => {
                for (i, (_, _, to)) in ready.iter().enumerate() {
                    self.files[ready[i].0].path = to.clone();
                }
                self.undo = pairs.iter().map(|(from, to)| (to.clone(), from.clone())).collect();
                self.message = if self.language == Language::Italian {
                    format!("Rinominati {} file.", pairs.len())
                } else {
                    format!("Renamed {} file(s).", pairs.len())
                };
            }
            Err((error, locations)) => {
                let mut undo = Vec::new();
                for (i, (file_index, from, _)) in ready.iter().enumerate() {
                    let location = locations.get(i).cloned().unwrap_or_else(|| from.clone());
                    self.files[*file_index].path = location.clone();
                    self.files[*file_index].status = "Rename failed".to_string();
                    if location != *from {
                        undo.push((location, from.clone()));
                    }
                }
                self.undo = undo;
                self.message = format!("{}: {}", self.language.text("Rename failed"), error);
            }
        }
        self.files_dirty = true;
        self.snapshot.clear();
    }

    fn undo_last(&mut self) {
        if self.undo.is_empty() {
            self.message = self.language.text("Nothing to undo!").to_string();
            return;
        }
        let batch = self.undo.clone();
        let pairs = batch.clone();
        let total = pairs.len();
        let file_indices: Vec<Option<usize>> = pairs
            .iter()
            .map(|(now, _)| self.files.iter().position(|f| f.path == *now))
            .collect();
        match rename_batch(&pairs) {
            Ok(()) => {
                for (i, (_, before)) in pairs.iter().enumerate() {
                    if let Some(file_index) = file_indices[i] {
                        self.files[file_index].path = before.clone();
                    }
                }
                self.undo.clear();
                self.message = if self.language == Language::Italian {
                    format!("Annullamento: ripristinati {} file su {}.", total, total)
                } else {
                    format!("Undo: {} out of {} file(s) restored.", total, total)
                };
            }
            Err((error, locations)) => {
                let mut retry = Vec::new();
                for (i, (now, before)) in pairs.iter().enumerate() {
                    let location = locations.get(i).cloned().unwrap_or_else(|| now.clone());
                    if let Some(file_index) = file_indices[i] {
                        self.files[file_index].path = location.clone();
                    }
                    if location != *before {
                        retry.push((location, before.clone()));
                    }
                }
                self.undo = retry;
                self.message = self.language.text("Undo failed: {}").replace("{}", &error);
            }
        }
        self.files_dirty = true;
    }
}

fn choices(ui: &mut egui::Ui, value: &mut usize, labels: &[&str]) {
    ui.horizontal_wrapped(|ui| {
        for (i, l) in labels.iter().enumerate() {
            ui.radio_value(value, i, *l);
        }
    });
}

fn text_field(ui: &mut egui::Ui, text: &mut String) -> egui::Response {
    ui.add(egui::TextEdit::singleline(text).desired_width(f32::INFINITY))
}

fn rule_editor(ui: &mut egui::Ui, r: &mut Rule, language: Language) {
    match r.kind {
        Kind::Insert => {
            ui.horizontal(|ui| {
                ui.label(language.text("Text:"));
                text_field(ui, &mut r.text1);
            });
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("Prefix"),
                    language.text("Suffix"),
                    language.text("At position"),
                ],
            );
            if r.choice == 2 {
                ui.horizontal(|ui| {
                    ui.label(language.text("Position:"));
                    ui.add(egui::DragValue::new(&mut r.n1).clamp_range(0..=1000));
                    ui.checkbox(&mut r.flag1, language.text("Count from the right"));
                });
            }
        }
        Kind::Delete => {
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("By position"),
                    language.text("Between texts"),
                ],
            );
            if r.choice == 0 {
                ui.horizontal(|ui| {
                    ui.label(language.text("From:"));
                    ui.add(egui::DragValue::new(&mut r.n1).clamp_range(0..=1000));
                    ui.label(language.text("Count:"));
                    ui.add(egui::DragValue::new(&mut r.n2).clamp_range(0..=1000));
                    ui.checkbox(&mut r.flag1, language.text("From the right"));
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label(language.text("From text:"));
                    text_field(ui, &mut r.text1);
                });
                ui.horizontal(|ui| {
                    ui.label(language.text("Until text:"));
                    text_field(ui, &mut r.text2);
                });
                ui.checkbox(&mut r.flag1, language.text("Keep the two texts"));
            }
        }
        Kind::Remove | Kind::Replace => {
            ui.horizontal(|ui| {
                ui.label(language.text("Find:"));
                text_field(ui, &mut r.text1);
            });
            if r.kind == Kind::Replace {
                ui.horizontal(|ui| {
                    ui.label(language.text("With:"));
                    text_field(ui, &mut r.text2);
                });
            }
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("First"),
                    language.text("All"),
                    language.text("Last"),
                ],
            );
            ui.checkbox(&mut r.flag1, language.text("Case sensitive"));
        }
        Kind::Extension => {
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("Set to"),
                    language.text("Lowercase"),
                    language.text("Uppercase"),
                    language.text("Remove"),
                    language.text("Add if missing"),
                ],
            );
            if r.choice == 0 || r.choice == 4 {
                ui.horizontal(|ui| {
                    ui.label(language.text("Extension:"));
                    text_field(ui, &mut r.text1);
                });
            }
        }
        Kind::Strip => {
            ui.checkbox(&mut r.flag1, language.text("Digits"));
            ui.checkbox(&mut r.flag2, language.text("Letters"));
            ui.checkbox(&mut r.flag3, language.text("Symbols"));
            ui.horizontal(|ui| {
                ui.label(language.text("Also these characters:"));
                text_field(ui, &mut r.text1);
            });
        }
        Kind::Case => {
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("lowercase"),
                    language.text("UPPERCASE"),
                    language.text("Each Word"),
                    language.text("First letter"),
                    language.text("Invert"),
                ],
            );
        }
        Kind::Number => {
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("Before name"),
                    language.text("After name"),
                    language.text("Replace name"),
                ],
            );
            ui.horizontal(|ui| {
                ui.label(language.text("Start:"));
                ui.add(egui::DragValue::new(&mut r.n1));
                ui.label(language.text("Step:"));
                ui.add(egui::DragValue::new(&mut r.n2));
                ui.label(language.text("Digits:"));
                ui.add(egui::DragValue::new(&mut r.n3).clamp_range(0..=20));
            });
            ui.horizontal(|ui| {
                ui.label(language.text("Separator:"));
                text_field(ui, &mut r.text1);
            });
        }
        Kind::Numbering => {
            ui.horizontal(|ui| {
                ui.label(language.text("Order:"));
                ui.radio_value(&mut r.flag1, false, language.text("Ascending"));
                ui.radio_value(&mut r.flag1, true, language.text("Descending"));
            });
            if r.flag1 {
                ui.small(language.text("Descending: the first file gets the highest number."));
            }
            ui.horizontal(|ui| {
                ui.label(language.text("Start:"));
                ui.add(egui::DragValue::new(&mut r.n1));
                ui.label(language.text("Step:"));
                ui.add(egui::DragValue::new(&mut r.n2).clamp_range(1..=1_000_000));
                ui.label(language.text("Digits:"));
                ui.add(egui::DragValue::new(&mut r.n3).clamp_range(0..=20));
            });
            ui.horizontal(|ui| {
                ui.label(language.text("Text before the number:"));
                text_field(ui, &mut r.text1);
            });
            ui.horizontal(|ui| {
                ui.label(language.text("Text after the number:"));
                text_field(ui, &mut r.text2);
            });
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("Before name"),
                    language.text("After name"),
                    language.text("Replace name"),
                ],
            );
        }
        Kind::Randomize => {
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("Before name"),
                    language.text("After name"),
                    language.text("Replace name"),
                ],
            );
            ui.horizontal(|ui| {
                ui.label(language.text("Length:"));
                ui.add(egui::DragValue::new(&mut r.n1).clamp_range(1..=200));
                ui.label(language.text("Seed:"));
                ui.add(egui::DragValue::new(&mut r.n2));
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut r.flag1, "0-9");
                ui.checkbox(&mut r.flag2, "a-z");
                ui.checkbox(&mut r.flag3, "A-Z");
            });
            ui.horizontal(|ui| {
                ui.label(language.text("Separator:"));
                text_field(ui, &mut r.text1);
            });
        }
        Kind::CleanUp => {
            ui.checkbox(&mut r.flag1, language.text("Trim spaces at both ends"));
            ui.checkbox(&mut r.flag2, language.text("Underscores to spaces"));
            ui.checkbox(&mut r.flag3, language.text("Collapse repeated spaces"));
            ui.checkbox(&mut r.flag4, language.text("Remove text in brackets"));
        }
        Kind::Regex => {
            ui.horizontal(|ui| {
                ui.label(language.text("Pattern:"));
                text_field(ui, &mut r.text1);
            });
            ui.horizontal(|ui| {
                ui.label(language.text("Replace with:"));
                text_field(ui, &mut r.text2);
            });
            choices(
                ui,
                &mut r.choice,
                &[
                    language.text("First match"),
                    language.text("All matches"),
                ],
            );
            if !r.text1.is_empty() && regex::Regex::new(&r.text1).is_err() {
                ui.colored_label(
                    egui::Color32::from_rgb(200, 60, 50),
                    language.text("Invalid expression"),
                );
            }
            ui.checkbox(&mut r.flag2, language.text("Ignore case"));
            ui.small(language.text("Use ${1}, ${2} ... for groups."));
        }
        Kind::Rearrange => {
            ui.horizontal(|ui| {
                ui.label(language.text("Split at any of:"));
                text_field(ui, &mut r.text1);
            });
            ui.horizontal(|ui| {
                ui.label(language.text("New pattern:"));
                text_field(ui, &mut r.text2);
            });
            ui.small(language.text("{1} is the first part, {2} the second, {0} the whole name."));
        }
        Kind::Padding => {
            choices(
                ui,
                &mut r.choice,
                &[language.text("Add zeros"), language.text("Remove zeros")],
            );
            ui.horizontal(|ui| {
                ui.label(language.text("Width:"));
                ui.add(egui::DragValue::new(&mut r.n1).clamp_range(0..=50));
            });
        }
        Kind::Transliterate => {
            ui.small(language.text("Replaces accented letters with plain ones (for example é becomes e)."));
        }
        Kind::AssignNames => {
            ui.small(language.text("One new name per line; line 1 goes to file 1, and so on."));
            ui.add(egui::TextEdit::multiline(&mut r.text1).desired_rows(4).desired_width(f32::INFINITY));
        }
        Kind::MapNames => {
            ui.small(language.text("One pair per line: old name, then a Tab or =, then new name."));
            ui.add(egui::TextEdit::multiline(&mut r.text1).desired_rows(4).desired_width(f32::INFINITY));
        }
    }
    if r.kind != Kind::Extension {
        ui.checkbox(&mut r.skip_ext, language.text("Skip extension"));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frames += 1;
        apply_theme(ctx, self.theme);

        let hovering_files = ctx.input(|input| !input.raw.hovered_files.is_empty());
        let dropped_paths: Vec<PathBuf> = ctx.input(|input| {
            input
                .raw
                .dropped_files
                .iter()
                .filter_map(|file| file.path.clone())
                .collect()
        });
        if !dropped_paths.is_empty() {
            let added = self.add_dropped_paths(dropped_paths);
            self.set_added_message(added);
        }

        self.refresh();

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(self.language.text("Batch Renamer App"));
                ui.label(self.language.text("by CarfRip"));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::from_id_source("theme")
                        .selected_text(match self.theme {
                            ThemeMode::Light => self.language.text("Light"),
                            ThemeMode::Dark => self.language.text("Dark"),
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.theme,
                                ThemeMode::Light,
                                self.language.text("Light"),
                            );
                            ui.selectable_value(
                                &mut self.theme,
                                ThemeMode::Dark,
                                self.language.text("Dark"),
                            );
                        });
                    ui.label(self.language.text("Theme"));
                    let old_language = self.language;
                    egui::ComboBox::from_id_source("language")
                        .selected_text(match self.language {
                            Language::Italian => "Italiano",
                            Language::English => "English",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.language, Language::Italian, "Italiano");
                            ui.selectable_value(&mut self.language, Language::English, "English");
                        });
                    ui.label(self.language.text("Language"));
                    if old_language != self.language {
                        self.message = if self.files.is_empty() {
                            if self.language == Language::Italian {
                                "Trascina qui i file o usa i pulsanti in alto per iniziare.".to_string()
                            } else {
                                "Drop files or use the buttons above to get started.".to_string()
                            }
                        } else {
                            self.language
                                .text("Loaded {} file(s).")
                                .replace("{}", &self.files.len().to_string())
                        };
                        ctx.send_viewport_cmd(egui::ViewportCommand::Title(
                            self.language.text("Batch Renamer App").to_string(),
                        ));
                    }
                });
            });
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                if ui.button(self.language.text("Add files...")).clicked() {
                    if let Some(p) = rfd::FileDialog::new().pick_files() {
                        let added = self.add_paths(p);
                        self.set_added_message(added);
                    }
                }
                if ui.button(self.language.text("Add folder...")).clicked() {
                    if let Some(d) = rfd::FileDialog::new().pick_folder() {
                        let added = self.add_folder(&d, 0);
                        self.set_added_message(added);
                    }
                }
                ui.checkbox(&mut self.subfolders, self.language.text("Include subfolders"));
                ui.separator();
                if ui
                    .add_enabled(
                        !self.files.is_empty(),
                        egui::Button::new(self.language.text("Validate")),
                    )
                    .clicked()
                {
                    self.validate();
                }
                let (rename_fill, rename_text) = match self.theme {
                    ThemeMode::Light => (
                        egui::Color32::from_rgb(35, 132, 83),
                        egui::Color32::WHITE,
                    ),
                    ThemeMode::Dark => (
                        egui::Color32::from_rgb(99, 203, 145),
                        egui::Color32::from_rgb(14, 32, 21),
                    ),
                };
                let ready_count = self.files.iter().filter(|file| file.status == "Ready").count();
                if ui
                    .add_enabled(
                        ready_count > 0,
                        egui::Button::new(
                            egui::RichText::new(self.language.text("Rename")).color(rename_text),
                        )
                        .fill(rename_fill),
                    )
                    .clicked()
                {
                    self.rename();
                }
                if ui
                    .add_enabled(
                        !self.undo.is_empty(),
                        egui::Button::new(self.language.text("Undo last rename")),
                    )
                    .clicked()
                {
                    self.undo_last();
                }
                ui.separator();
                if ui
                    .add_enabled(!self.files.is_empty(), egui::Button::new(self.language.text("Mark all")))
                    .clicked()
                {
                    for f in self.files.iter_mut() {
                        f.checked = true;
                    }
                    self.files_dirty = true;
                }
                if ui
                    .add_enabled(!self.files.is_empty(), egui::Button::new(self.language.text("Unmark all")))
                    .clicked()
                {
                    for f in self.files.iter_mut() {
                        f.checked = false;
                    }
                    self.files_dirty = true;
                }
                if ui
                    .add_enabled(!self.files.is_empty(), egui::Button::new(self.language.text("Sort by name")))
                    .clicked()
                {
                    self.files.sort_by(|a, b| {
                        a.path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_lowercase())
                            .cmp(&b.path.file_name().map(|n| n.to_string_lossy().to_lowercase()))
                    });
                    self.files_dirty = true;
                }
                if ui
                    .add_enabled(!self.files.is_empty(), egui::Button::new(self.language.text("Clear list")))
                    .clicked()
                {
                    self.files.clear();
                    self.undo.clear();
                    self.files_dirty = true;
                    self.message = self.language.text("List cleared.").to_string();
                }
            });
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                let checked = self.files.iter().filter(|f| f.checked).count();
                if self.language == Language::Italian {
                    ui.label(egui::RichText::new(format!(
                        "{} file · {} selezionati · {} regole · {} modelli",
                        self.files.len(), checked, self.rules.len(), self.presets.len()
                    )).strong().color(primary_text_color(self.theme)));
                } else {
                    ui.label(egui::RichText::new(format!(
                        "{} files · {} marked · {} rules · {} presets",
                        self.files.len(), checked, self.rules.len(), self.presets.len()
                    )).strong().color(primary_text_color(self.theme)));
                }
                ui.separator();
                ui.label(&self.message);
            });
        });

        egui::SidePanel::right("rules")
            .resizable(true)
            .min_width(350.0)
            .default_width(405.0)
            .show(ctx, |ui| {
            ui.heading(self.language.text("Rules"));
            ui.horizontal_wrapped(|ui| {
                egui::ComboBox::from_id_source("newkind")
                    .height(600.0)
                    .selected_text(self.language.kind_label(self.new_kind))
                    .show_ui(ui, |ui| {
                        for k in Kind::ALL.iter() {
                            ui.selectable_value(
                                &mut self.new_kind,
                                *k,
                                self.language.kind_label(*k),
                            );
                        }
                    });
                if ui.button(self.language.text("Add rule")).clicked() {
                    self.rules.push(Rule::new(self.new_kind));
                }
                if ui.button(self.language.text("Remove all")).clicked() {
                    self.rules.clear();
                }
            });
            ui.separator();

            // Presets sit in a bottom panel so the rule list takes all the remaining height.
            egui::TopBottomPanel::bottom("presets_panel")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.add_space(4.0);
                    ui.heading(self.language.text("Presets"));
                    ui.horizontal(|ui| {
                        let field_width = (ui.available_width() - 140.0).max(100.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.preset_name)
                                .desired_width(field_width)
                                .hint_text(self.language.text("Preset name")),
                        );
                        if ui.button(self.language.text("Save current rules")).clicked()
                            && !self.preset_name.trim().is_empty()
                        {
                            let name = self.preset_name.trim().to_string();
                            let rules = self.rules.clone();
                            if let Some(p) = self.presets.iter_mut().find(|p| p.name == name) {
                                p.rules = rules;
                            } else {
                                self.presets.push(Preset { name, rules });
                            }
                            self.message = match save_presets(&self.presets) {
                                Ok(()) => self.language.text("Preset saved.").to_string(),
                                Err(e) => self.language.text("Could not save presets: {}").replace("{}", &e),
                            };
                        }
                    });
                    let mut load: Option<usize> = None;
                    let mut del: Option<usize> = None;
                    egui::ScrollArea::vertical()
                        .id_source("presets")
                        .max_height(100.0)
                        .show(ui, |ui| {
                        for (i, p) in self.presets.iter().enumerate() {
                            ui.horizontal(|ui| {
                                let rule_word = match (self.language, p.rules.len() == 1) {
                                    (Language::Italian, true) => "regola",
                                    (Language::Italian, false) => "regole",
                                    (Language::English, true) => "rule",
                                    (Language::English, false) => "rules",
                                };
                                ui.label(format!("{} ({} {})", p.name, p.rules.len(), rule_word));
                                if ui.small_button(self.language.text("Load")).clicked() {
                                    load = Some(i);
                                }
                                if ui.small_button(self.language.text("Delete")).clicked() {
                                    del = Some(i);
                                }
                            });
                        }
                    });
                    if let Some(i) = load {
                        self.rules = self.presets[i].rules.clone();
                    }
                    if let Some(i) = del {
                        self.presets.remove(i);
                        self.message = match save_presets(&self.presets) {
                            Ok(()) => self.language.text("Preset deleted.").to_string(),
                            Err(e) => self.language.text("Could not save presets: {}").replace("{}", &e),
                        };
                    }
                    if self.presets.is_empty() {
                        ui.small(self.language.text("No presets saved"));
                    }
                    ui.add_space(4.0);
                });

            let mut remove: Option<usize> = None;
            let mut up: Option<usize> = None;
            let mut down: Option<usize> = None;
            let mut dup: Option<usize> = None;
            let count = self.rules.len();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                if self.rules.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(egui::RichText::new(self.language.text("No rules"))
                            .strong().color(primary_text_color(self.theme)));
                        ui.small(self.language.text("Add a rule to build your rename"));
                    });
                }
                for (i, r) in self.rules.iter_mut().enumerate() {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal_wrapped(|ui| {
                            ui.checkbox(&mut r.enabled, "");
                            ui.label(egui::RichText::new(format!("{}. {}", i + 1, self.language.kind_label(r.kind)))
                                .strong().color(primary_text_color(self.theme)));
                            if ui
                                .add_enabled(i > 0, egui::Button::new(self.language.text("Up")).small())
                                .clicked()
                            {
                                up = Some(i);
                            }
                            if ui
                                .add_enabled(i + 1 < count, egui::Button::new(self.language.text("Down")).small())
                                .clicked()
                            {
                                down = Some(i);
                            }
                            if ui.small_button(self.language.text("Copy")).clicked() {
                                dup = Some(i);
                            }
                            if ui.small_button(self.language.text("Delete")).clicked() {
                                remove = Some(i);
                            }
                        });
                        rule_editor(ui, r, self.language);
                    });
                }
            });
            if let Some(i) = up {
                self.rules.swap(i, i - 1);
            }
            if let Some(i) = down {
                self.rules.swap(i, i + 1);
            }
            if let Some(i) = dup {
                let c = self.rules[i].clone();
                self.rules.insert(i + 1, c);
            }
            if let Some(i) = remove {
                self.rules.remove(i);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(self.language.text("Preview"));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(
                        "{} {}",
                        self.files.iter().filter(|f| f.status == "Ready").count(),
                        self.language.text("Ready plural")
                    ));
                });
            });
            ui.separator();

            if hovering_files {
                egui::Frame::group(ui.style()).fill(match self.theme {
                    ThemeMode::Light => egui::Color32::from_rgb(222, 241, 229),
                    ThemeMode::Dark => egui::Color32::from_rgb(48, 76, 60),
                }).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(self.language.text("Drop files or folders here"))
                            .strong().color(primary_text_color(self.theme)));
                        ui.small(self.language.text("Drag files here to add them"));
                    });
                });
                ui.add_space(8.0);
            }

            if self.files.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(70.0);
                    ui.heading(self.language.text("No files added"));
                    ui.label(self.language.text("or use the buttons above to get started"));
                    ui.add_space(12.0);
                    if ui.button(self.language.text("Add files...")).clicked() {
                        if let Some(paths) = rfd::FileDialog::new().pick_files() {
                            let added = self.add_paths(paths);
                            self.set_added_message(added);
                        }
                    }
                    if ui.button(self.language.text("Add folder...")).clicked() {
                        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                            let added = self.add_folder(&folder, 0);
                            self.set_added_message(added);
                        }
                    }
                });
            } else {
                egui::ScrollArea::both().show(ui, |ui| {
                    egui::Grid::new("files")
                        .striped(true)
                        .num_columns(5)
                        .spacing(egui::vec2(14.0, 8.0))
                        .show(ui, |ui| {
                            ui.strong("");
                            let header = |text| egui::RichText::new(text)
                                .strong().color(primary_text_color(self.theme));
                            ui.label(header(self.language.text("Name")));
                            ui.label(header(self.language.text("New name")));
                            ui.label(header(self.language.text("Status")));
                            ui.label(header(self.language.text("Folder")));
                            ui.end_row();
                            let mut changed = false;
                            for f in self.files.iter_mut() {
                                if ui.checkbox(&mut f.checked, "").changed() {
                                    changed = true;
                                }
                                let old = f.path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                                ui.label(old);
                                ui.label(&f.new_name);
                                ui.colored_label(status_color(&f.status, self.theme), localized_status(self.language, &f.status));
                                let folder = f.path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                                ui.label(&folder).on_hover_text(folder);
                                ui.end_row();
                            }
                            if changed {
                                self.files_dirty = true;
                            }
                        });
                });
            }
        });

        // Edits made during this frame must show up without waiting for another input event.
        if self.refresh() {
            ctx.request_repaint();
        }

        // screenshot mode
        if self.screenshot_path.is_some() {
            if self.frames == 90 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
            }
            let mut shot: Option<std::sync::Arc<egui::ColorImage>> = None;
            ctx.input(|i| {
                for e in &i.events {
                    if let egui::Event::Screenshot { image, .. } = e {
                        shot = Some(image.clone());
                    }
                }
            });
            if let Some(img) = shot {
                let w = img.size[0] as u32;
                let h = img.size[1] as u32;
                let mut raw: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);
                for p in img.pixels.iter() {
                    raw.push(p.r());
                    raw.push(p.g());
                    raw.push(p.b());
                    raw.push(p.a());
                }
                if let (Some(buf), Some(path)) = (image::RgbaImage::from_raw(w, h, raw), self.screenshot_path.clone()) {
                    let _ = buf.save(path);
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = std::env::args().collect();
    let mut shot: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--screenshot" && i + 1 < args.len() {
            shot = Some(args[i + 1].clone());
            i += 1;
        }
        i += 1;
    }
    let icon = image::load_from_memory(include_bytes!("../assets/icon-1024.png"))
        .expect("assets/icon-1024.png must be a valid image")
        .into_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(Language::Italian.text("Batch Renamer App"))
            .with_icon(egui::IconData {
                rgba: icon.into_raw(),
                width: icon_width,
                height: icon_height,
            })
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([980.0, 620.0]),
        ..Default::default()
    };
    eframe::run_native(
        Language::Italian.text("Batch Renamer App"),
        options,
        Box::new(move |cc| {
            // Follow the Windows light/dark setting; fall back to light if unknown.
            let theme = match cc.integration_info.system_theme {
                Some(eframe::Theme::Dark) => ThemeMode::Dark,
                _ => ThemeMode::Light,
            };
            Box::new(App::new(shot.clone(), theme))
        }),
    )
}

#[cfg(test)]
mod rename_tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "batch-renamer-test-{}-{}",
            std::process::id(),
            nonce
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn rename_batch_supports_chains() {
        let dir = temp_dir();
        let a = dir.join("a.txt");
        let b = dir.join("b.txt");
        let c = dir.join("c.txt");
        std::fs::write(&a, "from a").unwrap();
        std::fs::write(&b, "from b").unwrap();

        rename_batch(&[(a.clone(), b.clone()), (b.clone(), c.clone())]).unwrap();

        assert_eq!(std::fs::read_to_string(&b).unwrap(), "from a");
        assert_eq!(std::fs::read_to_string(&c).unwrap(), "from b");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn rename_batch_supports_swaps_and_undo() {
        let dir = temp_dir();
        let a = dir.join("a.txt");
        let b = dir.join("b.txt");
        std::fs::write(&a, "from a").unwrap();
        std::fs::write(&b, "from b").unwrap();
        let batch = vec![(a.clone(), b.clone()), (b.clone(), a.clone())];

        rename_batch(&batch).unwrap();
        assert_eq!(std::fs::read_to_string(&a).unwrap(), "from b");
        assert_eq!(std::fs::read_to_string(&b).unwrap(), "from a");

        let undo: Vec<(PathBuf, PathBuf)> = batch
            .iter()
            .map(|(from, to)| (to.clone(), from.clone()))
            .collect();
        rename_batch(&undo).unwrap();
        assert_eq!(std::fs::read_to_string(&a).unwrap(), "from a");
        assert_eq!(std::fs::read_to_string(&b).unwrap(), "from b");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn unmarked_file_still_blocks_its_name() {
        let dir = temp_dir();
        let a = dir.join("alpha.txt");
        let b = dir.join("beta.txt");
        std::fs::write(&a, "alpha").unwrap();
        std::fs::write(&b, "beta").unwrap();
        let mut rule = Rule::new(Kind::MapNames);
        rule.text1 = "alpha=beta".to_string();
        let mut app = App {
            files: vec![
                Entry {
                    path: a,
                    checked: true,
                    new_name: String::new(),
                    status: String::new(),
                },
                Entry {
                    path: b,
                    checked: false,
                    new_name: String::new(),
                    status: String::new(),
                },
            ],
            rules: vec![rule],
            presets: Vec::new(),
            new_kind: Kind::Insert,
            preset_name: String::new(),
            message: String::new(),
            undo: Vec::new(),
            subfolders: false,
            snapshot: String::new(),
            files_dirty: true,
            screenshot_path: None,
            frames: 0,
            language: Language::Italian,
            theme: ThemeMode::Light,
        };

        app.recompute();

        assert_eq!(app.files[0].status, "Already exists");
        assert_eq!(app.files[1].status, "Skipped (unmarked)");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
