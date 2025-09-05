use crate::utils;
use eframe::egui;
use serde::{Deserialize, Serialize};

/// Egui application that wires the UI to the calculation and CLI parsing helpers.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MyEguiApp {
    // Inputs
    per_hour: f64,
    worked_hours: f32,
    increase_for_rate: f64,
    increase_rate_in_hours: f32,
    already_paid: f64,

    // CLI single-line input (eg: "*50 40")
    cli_line: String,

    // Optional free-form note field
    note: String,

    // UI state
    last_summary: Option<String>,
    last_error: Option<String>,
    // Calculated results
    total_earned: Option<f64>,
    remaining_to_pay: Option<f64>,

    // small helper to toggle showing advanced options
    show_advanced: bool,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Attempt to restore persisted state from `cc.storage` (requires the "persistence" feature).
        // If a saved state exists, return it; otherwise, fall back to defaults.
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value::<Self>(storage, eframe::APP_KEY) {
                return state;
            }
        }

        Self {
            per_hour: 50.0,
            worked_hours: 40.0,
            increase_for_rate: 10.0,
            increase_rate_in_hours: 10.0,
            already_paid: 0.0,
            cli_line: String::new(),
            note: String::new(),
            last_summary: None,
            last_error: None,
            total_earned: None,
            remaining_to_pay: None,
            show_advanced: false,
        }
    }

    fn reset_to_defaults(&mut self) {
        self.per_hour = 50.0;
        self.worked_hours = 40.0;
        self.increase_for_rate = 10.0;
        self.increase_rate_in_hours = 10.0;
        self.already_paid = 0.0;
        self.cli_line.clear();
        self.note.clear();
        self.last_summary = None;
        self.last_error = None;
        self.total_earned = None;
        self.remaining_to_pay = None;
        self.show_advanced = false;
    }

    fn validate_inputs(&self) -> Result<(), String> {
        if self.per_hour < 0.0 {
            return Err("Le taux horaire doit être positif".into());
        }
        if self.worked_hours < 0.0 {
            return Err("Le nombre d'heures travaillées doit être positif".into());
        }
        if self.increase_for_rate < 0.0 {
            return Err("L'augmentation de taux doit être positive ou nulle".into());
        }
        if self.increase_rate_in_hours <= 0.0 {
            return Err("Le nombre d'heures par palier doit être strictement positif".into());
        }
        if self.already_paid < 0.0 {
            return Err("Le montant déjà payé ne peut pas être négatif".into());
        }
        Ok(())
    }

    fn build_input_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("Entrées capturées :\n");
        s.push_str(&format!("• Taux de départ : {:.2} €/h\n", self.per_hour));
        s.push_str(&format!(
            "• Heures travaillées totales : {:.2} h\n",
            self.worked_hours
        ));
        s.push_str(&format!(
            "• Augmentation par palier : +{:.2} €/h\n",
            self.increase_for_rate
        ));
        s.push_str(&format!(
            "• Heures par palier : {:.2} h\n",
            self.increase_rate_in_hours
        ));
        s.push_str(&format!("• Déjà payé : {:.2} €\n", self.already_paid));
        if !self.note.trim().is_empty() {
            s.push_str("\nNotes utilisateur :\n");
            s.push_str(&format!("{}\n", self.note.trim()));
        }
        s
    }

    /// Try to parse a CLI style single-line input using the helpers in `utils`.
    /// On success, populate `per_hour` and `worked_hours`.
    fn apply_cli_line(&mut self) -> Result<(), String> {
        let line = self.cli_line.trim();
        if line.is_empty() {
            return Err("La ligne CLI est vide.".into());
        }

        // Call utils::get_value and get_number_of_hours which operate on &str and return Result
        match utils::get_value(line) {
            Ok(per) => match utils::get_number_of_hours(line) {
                Ok(hours) => {
                    self.per_hour = per;
                    self.worked_hours = hours;
                    self.last_error = None;
                    Ok(())
                }
                Err(e) => {
                    let msg = format!("Erreur parsing heures depuis CLI: {}", e);
                    self.last_error = Some(msg.clone());
                    Err(msg)
                }
            },
            Err(e) => {
                let msg = format!("Erreur parsing taux depuis CLI: {}", e);
                self.last_error = Some(msg.clone());
                Err(msg)
            }
        }
    }

    /// Run the business calculation by calling into `utils::calculate_payment`
    /// and update UI state with results.
    fn run_calculation(&mut self) -> Result<(), String> {
        self.validate_inputs()?;

        // Call the existing logic in utils to compute the total earned
        let total = utils::calculate_payment(
            self.per_hour,
            self.worked_hours,
            self.increase_for_rate,
            self.increase_rate_in_hours,
        );

        // Compute remaining to pay
        let remaining = total - self.already_paid;
        self.total_earned = Some(total);
        self.remaining_to_pay = Some(remaining);

        // Build a nice summary combining inputs and results
        let mut summary = self.build_input_summary();
        summary.push_str("\nRésultats :\n");
        summary.push_str(&format!("• Total gagné : {:.2} €\n", total));
        summary.push_str(&format!("• Déjà payé : {:.2} €\n", self.already_paid));
        summary.push_str(&format!("• Reste à payer : {:.2} €\n", remaining));
        self.last_summary = Some(summary);
        self.last_error = None;

        Ok(())
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use egui::{Align, Layout};

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("PayCalc");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.add(egui::Button::new("Quitter")).clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });

        egui::SidePanel::left("inputs_panel")
            .resizable(true)
            .min_width(320.0)
            .show(ctx, |ui| {
                ui.heading("Paramètres");
                ui.add_space(6.0);

                ui.label("Taux horaire de départ (€/h)");
                ui.add(egui::DragValue::new(&mut self.per_hour).speed(1.0).prefix("€ "));

                ui.separator();

                ui.label("Heures travaillées (total)");
                ui.add(egui::DragValue::new(&mut self.worked_hours).speed(0.5).suffix(" h"));

                ui.separator();

                ui.label("Augmentation du taux après chaque palier (+€/h)");
                ui.add(
                    egui::DragValue::new(&mut self.increase_for_rate)
                        .speed(1.0)
                        .prefix("€ "),
                );

                ui.label("Heures par palier (ex : 10 => augmentation toutes les 10h)");
                ui.add(
                    egui::DragValue::new(&mut self.increase_rate_in_hours)
                        .speed(0.5)
                        .suffix(" h"),
                );

                ui.separator();

                ui.label("Montant déjà payé (€)");
                ui.add(egui::DragValue::new(&mut self.already_paid).speed(1.0).prefix("€ "));

                ui.add_space(6.0);

                ui.collapsing("Ligne CLI (ex: *50 40)", |ui| {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.cli_line);
                        if ui.button("Appliquer").clicked() {
                            if let Err(e) = self.apply_cli_line() {
                                self.last_error = Some(e);
                                self.last_summary = None;
                                self.total_earned = None;
                                self.remaining_to_pay = None;
                            } else {
                                // successful parse - clear previous results
                                self.last_error = None;
                                self.total_earned = None;
                                self.remaining_to_pay = None;
                                self.last_summary = Some("Ligne CLI appliquée aux champs.".into());
                            }
                        }
                    });

                    ui.label("Vous pouvez coller la saisie CLI au format attendu par le programme.");
                });

                ui.add_space(8.0);

                ui.toggle_value(&mut self.show_advanced, "Afficher options avancées");
                if self.show_advanced {
                    ui.collapsing("Options avancées", |ui| {
                        ui.label("Note: vous pouvez utiliser ce champ pour ajouter un commentaire.");
                        ui.text_edit_multiline(&mut self.note);
                    });
                }

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Calculer").clicked() {
                        if let Err(e) = self.run_calculation() {
                            self.last_error = Some(e);
                            self.last_summary = None;
                            self.total_earned = None;
                            self.remaining_to_pay = None;
                        }
                    }

                    if ui.button("Réinitialiser").clicked() {
                        self.reset_to_defaults();
                    }
                });

                ui.add_space(6.0);

                ui.label(egui::RichText::new(
                    "Remarque: les entrées sont validées. Le calcul est effectué en appelant `utils::calculate_payment`.",
                )
                .small());
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Aperçu & Résultats");
            });

            ui.add_space(8.0);

            ui.group(|ui| {
                ui.label(egui::RichText::new("Résumé des entrées").strong());
                ui.add_space(6.0);

                ui.columns(2, |cols| {
                    let (left_slice, right_slice) = cols.split_at_mut(1);
                    let left = &mut left_slice[0];
                    let right = &mut right_slice[0];

                    left.label(format!("Taux de départ: {:.2} €/h", self.per_hour));
                    right.label(format!("Heures totales: {:.2} h", self.worked_hours));

                    left.label(format!("Augmentation par palier: +{:.2} €/h", self.increase_for_rate));
                    right.label(format!("Heures par palier: {:.2} h", self.increase_rate_in_hours));

                    left.label(format!("Déjà payé: {:.2} €", self.already_paid));
                    right.label(format!("Note: {}", if self.note.is_empty() { "—" } else { &self.note }));
                });

                ui.add_space(8.0);

                ui.separator();
                ui.add_space(6.0);

                ui.label(egui::RichText::new("Résultats calculés").strong());
                ui.add_space(6.0);

                if let Some(total) = self.total_earned {
                    ui.label(format!("Total gagné : {:.2} €", total));
                } else {
                    ui.label("Total gagné : —");
                }

                ui.label(format!("Déjà payé : {:.2} €", self.already_paid));

                if let Some(remaining) = self.remaining_to_pay {
                    ui.label(format!("Reste à payer : {:.2} €", remaining));
                } else {
                    ui.label("Reste à payer : —");
                }

                ui.add_space(6.0);

                if let Some(err) = &self.last_error {
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, format!("Erreur : {}", err));
                } else if let Some(summary) = &self.last_summary {
                    ui.separator();
                    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        ui.label(summary);
                    });
                } else {
                    ui.separator();
                    ui.label("Appuyez sur 'Calculer' pour exécuter la logique et afficher les résultats.");
                }
            });

            ui.add_space(12.0);

            ui.collapsing("Instructions pour l'intégration", |ui| {
                ui.label("- Le calcul utilise désormais `utils::calculate_payment`.");
                ui.label("- `calculate_payment` retourne le total gagné (f64).");
                ui.label("- Le champ 'Déjà payé' est utilisé pour calculer le reste à payer.");
                ui.add_space(6.0);
                ui.label("Conseil: si vous souhaitez afficher un détail par palier, adaptez `utils::calculate_payment` pour renvoyer un breakdown structure (par ex. Vec<(hours, rate, amount)>) et consommez-le ici.");
            });
        });

        // Small floating help on the right
        egui::SidePanel::right("help_panel")
            .resizable(false)
            .width_range(220.0..=320.0)
            .show(ctx, |ui| {
                ui.heading("Aide rapide");
                ui.separator();
                ui.label("Saisissez ou collez les valeurs dans le panneau de gauche.");
                ui.label("Utilisez le bouton 'Appliquer' dans la section CLI pour parser une ligne au format `*<rate> <hours>`.");
                ui.add_space(6.0);
                ui.label("Conseils :");
                ui.label("• Les champs acceptent des valeurs décimales.");
                ui.label("• Vérifiez les unités (€/h et h).");
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Persist the entire app state. This requires `MyEguiApp` to implement
        // `serde::Serialize` (done via the derive above) and eframe to be built
        // with the `persistence` feature (Cargo.toml already includes it).
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
