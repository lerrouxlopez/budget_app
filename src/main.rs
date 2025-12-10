#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::egui;
use egui::{Color32, RichText};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let viewport = if let Some(icon) = app_icon() {
        egui::ViewportBuilder::default().with_icon(icon)
    } else {
        egui::ViewportBuilder::default()
    };
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "DYBudget",
        options,
        Box::new(|_cc| Box::new(BudgetApp::default())),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    title: String,
    date: String,
    amount: f64,
    color: Color32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedState {
    monthly_budget: f64,
    transactions: Vec<Transaction>,
}

#[derive(Debug)]
struct BudgetApp {
    monthly_budget: f64,
    budget_input: String,
    entry_description: String,
    entry_amount: String,
    transactions: Vec<Transaction>,
    status: String,
    show_new_tx: bool,
    form_title: String,
    form_amount: String,
    form_date: String,
}

impl Default for BudgetApp {
    fn default() -> Self {
        let mut app = Self {
            monthly_budget: 2500.0,
            budget_input: "2500".to_string(),
            entry_description: String::new(),
            entry_amount: String::new(),
            transactions: vec![],
            status: String::new(),
            show_new_tx: false,
            form_title: String::new(),
            form_amount: String::new(),
            form_date: "Today".to_string(),
        };
        if let Some(saved) = app.load_state() {
            app.monthly_budget = saved.monthly_budget;
            app.budget_input = format!("{:.2}", saved.monthly_budget);
            app.transactions = saved.transactions;
        } else {
            // Seed a few to match the mock.
            app.transactions = vec![
                Transaction {
                    title: "teva overflow".to_string(),
                    date: "Aug 31, 2023".to_string(),
                    amount: -458.00,
                    color: Color32::from_rgb(42, 201, 121),
                },
                Transaction {
                    title: "mcdonald".to_string(),
                    date: "Aug 30, 2023".to_string(),
                    amount: -119.46,
                    color: Color32::from_rgb(230, 78, 95),
                },
                Transaction {
                    title: "bath and bodyworks".to_string(),
                    date: "Aug 30, 2023".to_string(),
                    amount: -80.00,
                    color: Color32::from_rgb(110, 133, 255),
                },
                Transaction {
                    title: "dominos pizza".to_string(),
                    date: "Aug 28, 2023".to_string(),
                    amount: -81.00,
                    color: Color32::from_rgb(230, 156, 71),
                },
                Transaction {
                    title: "dr.locker".to_string(),
                    date: "Aug 28, 2023".to_string(),
                    amount: -40.00,
                    color: Color32::from_rgb(180, 180, 200),
                },
            ];
        }
        app
    }
}

impl BudgetApp {
    fn total_spent(&self) -> f64 {
        self.transactions.iter().map(|t| t.amount.abs()).sum()
    }

    fn remaining(&self) -> f64 {
        self.monthly_budget - self.total_spent()
    }

    fn update_budget_from_input(&mut self) {
        match self.budget_input.trim().parse::<f64>() {
            Ok(val) if val >= 0.0 => {
                self.monthly_budget = val;
                self.status = format!("Monthly budget set to ${:.2}", val);
                let _ = self.save_state();
            }
            _ => {
                self.status = "Enter a valid non-negative number for the monthly budget.".to_string();
            }
        }
    }

    fn add_entry(&mut self) {
        let amount = match self.entry_amount.trim().parse::<f64>() {
            Ok(val) if val > 0.0 => val,
            _ => {
                self.status = "Enter a positive number for the amount.".to_string();
                return;
            }
        };

        if self.entry_description.trim().is_empty() {
            self.status = "Describe the entry before adding it.".to_string();
            return;
        }

        self.transactions.push(Transaction {
            title: self.entry_description.trim().to_string(),
            date: "Today".to_string(),
            amount: -amount,
            color: Color32::from_rgb(88, 172, 255),
        });

        self.entry_description.clear();
        self.entry_amount.clear();
        self.status = "Entry added.".to_string();
        let _ = self.save_state();
    }

    fn apply_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(22.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        );
        ctx.set_style(style);

        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(23, 23, 30);
        visuals.extreme_bg_color = Color32::from_rgb(18, 18, 24);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(36, 36, 46);
        visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(52, 52, 64);
        visuals.widgets.active.bg_fill = Color32::from_rgb(78, 90, 255);
        visuals.widgets.active.fg_stroke.color = Color32::WHITE;
        visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(220, 225, 235);
        ctx.set_visuals(visuals);
    }

    fn nav_button(ui: &mut egui::Ui, label: &str, selected: bool) {
        let bg = if selected {
            Color32::from_rgb(50, 50, 60)
        } else {
            Color32::TRANSPARENT
        };
        let text = RichText::new(label)
            .color(Color32::from_rgb(220, 225, 235))
            .strong();
        egui::Frame::none()
            .fill(bg)
            .rounding(egui::Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(10.0, 8.0))
            .show(ui, |ui| {
                ui.label(text);
            });
    }

    fn stat_card(ui: &mut egui::Ui, title: &str, amount: f64, accent: Color32) {
        egui::Frame::none()
            .fill(Color32::from_rgb(35, 35, 45))
            .rounding(egui::Rounding::same(12.0))
            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(60, 60, 80)))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(title)
                        .color(Color32::from_rgb(190, 195, 210))
                        .size(13.0),
                );
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("PHP").color(Color32::from_rgb(190, 195, 210)));
                    ui.label(
                        RichText::new(format!("{:+.2}", amount))
                            .color(accent)
                            .size(20.0)
                            .strong(),
                    );
                });
            });
    }

    fn transaction_row(ui: &mut egui::Ui, tx: &Transaction) {
        egui::Frame::none()
            .fill(Color32::from_rgb(32, 32, 40))
            .rounding(egui::Rounding::same(10.0))
            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("●")
                            .color(tx.color)
                            .size(22.0)
                            .family(egui::FontFamily::Monospace),
                    );
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(tx.title.to_owned())
                                .color(Color32::from_rgb(220, 225, 235))
                                .strong(),
                        );
                        ui.label(
                            RichText::new(tx.date.to_owned())
                                .color(Color32::from_rgb(150, 155, 165))
                                .size(12.0),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let color = if tx.amount < 0.0 {
                            Color32::from_rgb(240, 80, 100)
                        } else {
                            Color32::from_rgb(110, 220, 140)
                        };
                        ui.label(
                            RichText::new(format!("{:+.2}", tx.amount))
                                .color(color)
                                .strong(),
                        );
                    });
                });
            });
    }

    fn data_path() -> PathBuf {
        PathBuf::from("budget_data.json")
    }

    fn save_state(&self) -> Result<(), String> {
        let state = PersistedState {
            monthly_budget: self.monthly_budget,
            transactions: self.transactions.clone(),
        };
        serde_json::to_string_pretty(&state)
            .map_err(|e| e.to_string())
            .and_then(|json| fs::write(Self::data_path(), json).map_err(|e| e.to_string()))
    }

    fn load_state(&self) -> Option<PersistedState> {
        fs::read_to_string(Self::data_path())
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
    }

    fn new_tx_form(&mut self, ctx: &egui::Context) {
        let mut open = self.show_new_tx;
        egui::Window::new("New transaction")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.set_width(360.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new("Add a new transaction").strong());
                    ui.add_space(8.0);
                    ui.label("Title");
                    ui.text_edit_singleline(&mut self.form_title);
                    ui.label("Date");
                    ui.text_edit_singleline(&mut self.form_date);
                    ui.label("Amount (use negative for expense, positive for income)");
                    ui.text_edit_singleline(&mut self.form_amount);
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui
                            .add(
                                egui::Button::new(RichText::new("Save").color(Color32::WHITE))
                                    .fill(Color32::from_rgb(92, 106, 255)),
                            )
                            .clicked()
                        {
                            let parsed_amount = self.form_amount.trim().parse::<f64>();
                            if self.form_title.trim().is_empty() {
                                self.status = "Enter a title for the transaction.".to_string();
                            } else if let Ok(amount) = parsed_amount {
                                self.transactions.push(Transaction {
                                    title: self.form_title.trim().to_string(),
                                    date: self.form_date.trim().to_string(),
                                    amount,
                                    color: if amount < 0.0 {
                                        Color32::from_rgb(230, 78, 95)
                                    } else {
                                        Color32::from_rgb(110, 220, 140)
                                    },
                                });
                                self.form_title.clear();
                                self.form_amount.clear();
                                self.form_date = "Today".to_string();
                                self.status = "Transaction saved.".to_string();
                                let _ = self.save_state();
                                self.show_new_tx = false;
                            } else {
                                self.status = "Enter a valid number for amount.".to_string();
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_new_tx = false;
                        }
                    });
                });
            });
        self.show_new_tx = open;
    }
}

fn app_icon() -> Option<Arc<egui::IconData>> {
    let png_bytes = include_bytes!("../assets/icon.png");
    eframe::icon_data::from_png_bytes(png_bytes)
        .ok()
        .map(Arc::new)
}

impl eframe::App for BudgetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_style(ctx);

        egui::SidePanel::left("nav")
            .exact_width(190.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("Spensieur")
                            .color(Color32::from_rgb(230, 232, 240))
                            .size(18.0)
                            .strong(),
                    );
                    ui.add_space(16.0);
                });

                Self::nav_button(ui, "Home", true);
                Self::nav_button(ui, "Accounts", false);
                Self::nav_button(ui, "Categories", false);

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Analytics")
                        .color(Color32::from_rgb(160, 165, 175))
                        .size(13.0),
                );
                Self::nav_button(ui, "Cashflow", false);
                Self::nav_button(ui, "Expenses", false);
                Self::nav_button(ui, "Income", false);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(12.0);
                    Self::nav_button(ui, "Options", false);
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(18, 18, 24))
                    .inner_margin(egui::Margin::symmetric(16.0, 14.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Dashboard")
                            .color(Color32::from_rgb(230, 232, 240))
                            .size(20.0)
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("New transaction")
                                        .color(Color32::WHITE)
                                        .strong(),
                                )
                                .fill(Color32::from_rgb(92, 106, 255))
                                .rounding(egui::Rounding::same(10.0)),
                            )
                            .clicked()
                        {
                            self.show_new_tx = true;
                        }
                    });
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width() * 0.5 - 6.0, 100.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            Self::stat_card(ui, "All Account (PHP)", -self.total_spent(), Color32::from_rgb(127, 138, 255));
                        },
                    );
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), 100.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            Self::stat_card(ui, "Bank (PHP)", -self.total_spent(), Color32::from_rgb(120, 255, 205));
                        },
                    );
                });

                ui.add_space(8.0);
                egui::Frame::none()
                    .fill(Color32::from_rgb(25, 25, 33))
                    .rounding(egui::Rounding::same(12.0))
                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(45, 45, 55)))
                    .inner_margin(egui::Margin::same(14.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("Monthly budget")
                                        .color(Color32::from_rgb(160, 165, 175)),
                                );
                                ui.label(
                                    RichText::new(format!("PHP {:.2}", self.monthly_budget))
                                        .color(Color32::from_rgb(220, 225, 235))
                                        .size(18.0)
                                        .strong(),
                                );
                            });
                            ui.add_space(12.0);
                            ui.vertical(|ui| {
                                ui.label(RichText::new("Remaining").color(Color32::from_rgb(160, 165, 175)));
                                ui.label(
                                    RichText::new(format!("PHP {:.2}", self.remaining()))
                                        .color(Color32::from_rgb(110, 220, 140))
                                        .strong(),
                                );
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Budget (PHP)").color(Color32::from_rgb(180, 185, 195)));
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.budget_input)
                                            .desired_width(100.0),
                                    );
                                    if ui.button("Update").clicked() {
                                        self.update_budget_from_input();
                                    }
                                });
                            });
                        });
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Add entry").color(Color32::from_rgb(180, 185, 195)));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.entry_description)
                                    .hint_text("Description")
                                    .desired_width(200.0),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut self.entry_amount)
                                    .hint_text("Amount")
                                    .desired_width(100.0),
                            );
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("Add")
                                            .color(Color32::WHITE)
                                            .strong(),
                                    )
                                    .fill(Color32::from_rgb(92, 106, 255))
                                    .rounding(egui::Rounding::same(8.0)),
                                )
                                .clicked()
                            {
                                self.add_entry();
                            }
                        });
                    });

                ui.add_space(12.0);
                ui.label(
                    RichText::new("Recent transactions:")
                        .color(Color32::from_rgb(220, 225, 235))
                        .size(16.0)
                        .strong(),
                );
                ui.add_space(6.0);
                egui::ScrollArea::vertical()
                    .max_height(320.0)
                    .show(ui, |ui| {
                        for tx in self.transactions.iter().rev() {
                            Self::transaction_row(ui, tx);
                            ui.add_space(6.0);
                        }
                    });

                if !self.status.is_empty() {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(&self.status)
                            .color(Color32::from_rgb(140, 180, 255))
                            .size(13.0),
                    );
                }
            });

        if self.show_new_tx {
            self.new_tx_form(ctx);
        }
    }
}
