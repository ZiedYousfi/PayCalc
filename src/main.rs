mod gui;
mod utils;

fn main() {
    println!("Hello, world!");

    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "PayCalc",
        options,
        Box::new(|cc| Ok(Box::new(gui::MyEguiApp::new(cc)))),
    );

    // let increase_for_rate = match utils::get_rate_increase() {
    //     Ok(rate) => rate,
    //     Err(e) => {
    //         eprintln!("Erreur lors de la saisie de l'augmentation de taux : {e}");
    //         0.0
    //     }
    // };

    // let increase_rate_in_hours = match utils::get_hours_for_increase() {
    //     Ok(hours) => hours,
    //     Err(e) => {
    //         eprintln!("Erreur lors de la saisie du nombre d'heures par palier : {e}");
    //         0.0
    //     }
    // };

    // println!(
    //     "\nTaux d'augmentation : +{increase_for_rate} après chaque {increase_rate_in_hours} heures."
    // );

    // let (per_hour, worked_hours) = match utils::get_wage_and_hours() {
    //     Ok(result) => result,
    //     Err(e) => {
    //         eprintln!(
    //             "Erreur lors de la saisie du salaire horaire et des heures travaillées : {e}"
    //         );
    //         (0.0, 0.0)
    //     }
    // };

    // println!("\nTaux de départ : {per_hour}€/h");
    // println!("Nombre total d'heures travaillées : {worked_hours}");

    // let to_pay = utils::calculate_payment(
    //     per_hour,
    //     worked_hours,
    //     increase_for_rate,
    //     increase_rate_in_hours,
    // );

    // let already_paid = match utils::get_already_paid_amount() {
    //     Ok(amount) => amount,
    //     Err(e) => {
    //         eprintln!("Erreur lors de la saisie du montant déjà payé : {e}");
    //         0.0
    //     }
    // };

    // let final_amount = to_pay - already_paid;

    // println!("\n🌸 Résumé 🌸");
    // println!("Total gagné : {to_pay:.2}€");
    // println!("Déjà payé : {already_paid:.2}€");
    // println!("Reste à payer : {final_amount:.2}€");
    // println!("Merci pour votre travail 💖");
}
