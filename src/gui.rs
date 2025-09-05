// A friendly egui-based UI that replaces the CLI inputs from the original program.
// This file intentionally does NOT implement any calculation logic — it only collects
// the inputs and presents them to the user. The user will plug the calculation logic.
//
// The UI collects:
//  - Starting hourly rate (`per_hour`)
//  - Total worked hours (`worked_hours`)
//  - Hourly increase amount per step (`increase_for_rate`)
//  - Number of hours per increase step (`increase_rate_in_hours`)
//  - Already paid amount (`already_paid`)
//
// The "Calculate" button validates inputs and shows a summary of captured values
// and placeholders where results would be displayed once calculation logic is added.

use eframe::egui;


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
#[derive(Default)]
pub struct MyEguiApp {
    // Inputs
    per_hour: f64,
    worked_hours: f32,
    increase_for_rate: f64,
    increase_rate_in_hours: f32,
    already_paid: f64,

    // Optional free-form note field
    note: String,

    // UI state
    last_summary: Option<String>,
    last_error: Option<String>,
    // small helper to toggle showing advanced options
    show_advanced: bool,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // You can use cc here to restore persisted state if desired.
        Self {
            // Set helpful defaults that make playing with the UI convenient.
            per_hour: 50.0,
            worked_hours: 40.0,
            increase_for_rate: 10.0,
            increase_rate_in_hours: 10.0,
            already_paid: 0.0,
            note: String::new(),
            last_summary: None,
            last_error: None,
            show_advanced: false,
        }
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

    fn make_summary(&self) -> String {
        // NOTE: this intentionally does not calculate totals. It only summarizes inputs.
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

        s.push_str("\nRemarques :\n");
        s.push_str("• Calcul non exécuté — veuillez implémenter la logique dans `utils::calculate_payment` et l'appeler ici.\n");
        s.push_str("• Une fois la logique ajoutée, remplacez l'espace réservé ci-dessous par les résultats calculés.\n");

        s
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with title
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("PayCalc");
                ui.label("— UI for capturing wage & hours (calculation not run)");
            });
        });

        // Left side: inputs & actions
        egui::SidePanel::left("inputs_panel")
            .resizable(true)
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Paramètres");
                ui.add_space(4.0);

                ui.label("Taux horaire de départ (€/h)");
                ui.add(
                    egui::DragValue::new(&mut self.per_hour)
                        .speed(1.0)
                        .prefix("€ "),
                );

                ui.separator();

                ui.label("Heures travaillées (total)");
                ui.add(
                    egui::DragValue::new(&mut self.worked_hours)
                        .speed(0.5)
                        .suffix(" h"),
                );

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
                ui.add(
                    egui::DragValue::new(&mut self.already_paid)
                        .speed(1.0)
                        .prefix("€ "),
                );

                ui.separator();

                ui.toggle_value(&mut self.show_advanced, "Afficher options avancées");

                if self.show_advanced {
                    ui.collapsing("Options avancées", |ui| {
                        ui.label("Note: vous pouvez utiliser ce champ pour ajouter un commentaire.");
                        ui.text_edit_multiline(&mut self.note);
                    });
                }

                ui.add_space(8.0);

                // Action buttons
                ui.horizontal(|ui| {
                    if ui.button("Calculer (non implémenté)").clicked() {
                        match self.validate_inputs() {
                            Ok(()) => {
                                self.last_error = None;
                                self.last_summary = Some(self.make_summary());
                            }
                            Err(e) => {
                                self.last_error = Some(e);
                            }
                        }
                    }

                    if ui.button("Réinitialiser").clicked() {
                        // Reset fields manually to the initial defaults (do not call CreationContext::default())
                        self.per_hour = 50.0;
                        self.worked_hours = 40.0;
                        self.increase_for_rate = 10.0;
                        self.increase_rate_in_hours = 10.0;
                        self.already_paid = 0.0;
                        self.note.clear();
                        self.last_summary = None;
                        self.last_error = None;
                        self.show_advanced = false;
                    }
                });

                ui.add_space(8.0);
                ui.label(egui::RichText::new(
                    "Les boutons n'exécutent pas la logique de calcul. Intégrez la fonction de calcul avant d'utiliser les résultats.",
                ).small());
            });

        // Central panel: preview & placeholders for results
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Aperçu & Résultats");
            });

            ui.add_space(6.0);

            // Preview area shows the raw captured inputs
            ui.group(|ui| {
                ui.label(egui::RichText::new("Résumé des entrées").strong());
                ui.add_space(6.0);

                ui.columns(2, |cols| {
                    // avoid mutably borrowing two elements from the same slice directly;
                    // use split_at_mut to obtain two non-overlapping mutable slices
                    let (left_slice, right_slice) = cols.split_at_mut(1);
                    let left = &mut left_slice[0];
                    let right = &mut right_slice[0];

                    left.label(format!("Taux de départ: {:.2} €/h", self.per_hour));
                    right.label(format!("Heures totales: {:.2} h", self.worked_hours));

                    left.label(format!(
                        "Augmentation par palier: +{:.2} €/h",
                        self.increase_for_rate
                    ));
                    right.label(format!(
                        "Heures par palier: {:.2} h",
                        self.increase_rate_in_hours
                    ));

                    left.label(format!("Déjà payé: {:.2} €", self.already_paid));
                    right.label(format!("Note: {}", if self.note.is_empty() { "—" } else { &self.note }));
                });

                ui.add_space(8.0);

                ui.separator();
                ui.add_space(6.0);

                // Placeholder result area
                ui.label(egui::RichText::new("Résultats (placeholders)").strong());
                ui.add_space(6.0);
                ui.label("Total gagné : — (calculation not implemented)");
                ui.label("Déjà payé : —");
                ui.label("Reste à payer : —");

                ui.add_space(6.0);

                // Show summary or errors below
                if let Some(err) = &self.last_error {
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, format!("Erreur : {}", err));
                } else if let Some(summary) = &self.last_summary {
                    ui.separator();
                    ui.label(egui::RichText::new("Résumé capturé").strong());
                    ui.add_space(4.0);
                    egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                        ui.label(summary);
                    });
                } else {
                    ui.separator();
                    ui.label("Appuyez sur 'Calculer (non implémenté)' pour générer un résumé des entrées.");
                }
            });

            ui.add_space(12.0);

            // Helpful footer / next steps for whoever implements the logic
            ui.collapsing("Instructions pour l'intégration", |ui| {
                ui.label("- Appelez `utils::calculate_payment(per_hour, worked_hours, increase_for_rate, increase_rate_in_hours)`");
                ui.label("- Utilisez le résultat retourné pour remplir 'Total gagné' et calculez 'Reste à payer'");
                ui.label("- Affichez les montants avec 2 décimales et des étiquettes claires");
                ui.add_space(6.0);
                ui.label("Remarque : la validation simple est déjà en place ; adaptez selon les besoins métiers.");
            });
        });

        // small top-right floating help
        egui::SidePanel::right("help_panel")
            .resizable(false)
            .width_range(200.0..=320.0)
            .show(ctx, |ui| {
                ui.heading("Aide rapide");
                ui.separator();
                ui.label("Saisissez les valeurs dans le panneau de gauche.");
                ui.label("Le bouton 'Calculer' ne lance pas encore la logique.");
                ui.add_space(6.0);
                ui.label("Conseils :");
                ui.label("• Utilisez des nombres positifs.");
                ui.label("• Vérifiez les unités (€/h et h).");
            });
    }
}
