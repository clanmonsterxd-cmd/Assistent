use eframe::egui;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};

pub struct AssistantApp {
    input: String,
    chat_history: Vec<ChatMessage>,
    is_active: bool,
    is_minimized: bool,
    tx: Sender<String>,
    rx: Arc<Mutex<Receiver<String>>>,
    backend_tx: Sender<String>,
}

#[derive(Clone)]
pub struct ChatMessage {
    pub is_user: bool,
    pub text: String,
}

impl AssistantApp {
    pub fn new(tx: Sender<String>, rx: Receiver<String>, backend_tx: Sender<String>) -> Self {
        Self {
            input: String::new(),
            chat_history: vec![
                ChatMessage {
                    is_user: false,
                    text: "Willkommen beim KI-Assistenten!\n\nSage 'Hallo' um mich zu aktivieren.\n\nIch kann dir helfen mit:\n• Wettervorhersagen\n• Wikipedia-Suchen\n• Programmen öffnen".to_string(),
                }
            ],
            is_active: false,
            is_minimized: false,
            tx,
            rx: Arc::new(Mutex::new(rx)),
            backend_tx,
        }
    }

    pub fn add_message(&mut self, is_user: bool, text: &str) {
        self.chat_history.push(ChatMessage {
            is_user,
            text: text.to_string(),
        });
    }

    fn send_input(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }

        let message = self.input.clone();
        self.add_message(true, &message);
        
        let lower = message.to_lowercase();
        if !self.is_active && (lower.contains("hallo") || lower.contains("hey") || lower.contains("hi")) {
            self.is_active = true;
        }
        
        if self.is_active && (lower.contains("danke") || lower.contains("tschüss") || lower.contains("bis")) {
            self.is_minimized = true;
            self.add_message(false, "Fenster wird minimiert. Sage 'Hallo' zum Reaktivieren.");
        }
        
        let _ = self.tx.send(message);
        self.input.clear();
    }

    pub fn check_responses(&mut self) {
        let mut responses = Vec::new();
        
        if let Ok(rx) = self.rx.try_lock() {
            while let Ok(response) = rx.try_recv() {
                responses.push(response);
            }
        }
        
        for response in responses {
            if response.contains("aktiviert") {
                self.is_active = true;
                self.is_minimized = false;
            }
            
            if response.contains("Bis bald") || response.contains("minimiert") {
                self.is_minimized = true;
            }
            
            self.add_message(false, &response);
        }
    }
}

impl eframe::App for AssistantApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_responses();

        if self.is_minimized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        }

        // Schöner dunkler Hintergrund
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = egui::Color32::from_rgb(25, 25, 35);
        style.visuals.panel_fill = egui::Color32::from_rgb(30, 30, 40);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(20, 20, 28);
        
        // Größere Schrift
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        );
        
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header mit Farbverlauf
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                
                ui.heading(
                    egui::RichText::new("KI-Assistent")
                        .size(28.0)
                        .color(egui::Color32::from_rgb(100, 180, 255))
                        .strong()
                );
                
                ui.add_space(5.0);
                
                let status_text = if self.is_active {
                    egui::RichText::new("* AKTIVIERT")
                        .size(14.0)
                        .color(egui::Color32::from_rgb(100, 255, 150))
                } else {
                    egui::RichText::new("o Warte auf Begrüßung")
                        .size(14.0)
                        .color(egui::Color32::from_rgb(255, 200, 100))
                };
                
                ui.label(status_text);
                
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(10.0);
            });

            // Chat-Bereich mit Scrolling
            let scroll_height = ui.available_height() - 90.0;
            
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(scroll_height)
                .show(ui, |ui| {
                    ui.add_space(5.0);
                    
                    for msg in &self.chat_history {
                        ui.horizontal_wrapped(|ui| {
                            if msg.is_user {
                                // User-Nachricht (rechts, blau)
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                    let text = egui::RichText::new(&msg.text)
                                        .size(15.0)
                                        .color(egui::Color32::WHITE);
                                    
                                    ui.add(
                                        egui::Label::new(text)
                                            .wrap()
                                    );
                                    
                                    ui.label(
                                        egui::RichText::new(">")
                                            .size(18.0)
                                            .color(egui::Color32::from_rgb(100, 180, 255))
                                    );
                                });
                            } else {
                                // Bot-Nachricht (links, grau)
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                    ui.label(
                                        egui::RichText::new("<")
                                            .size(18.0)
                                            .color(egui::Color32::from_rgb(150, 150, 200))
                                    );
                                    
                                    let text = egui::RichText::new(&msg.text)
                                        .size(15.0)
                                        .color(egui::Color32::from_rgb(220, 220, 230));
                                    
                                    ui.add(
                                        egui::Label::new(text)
                                            .wrap()
                                    );
                                });
                            }
                        });
                        ui.add_space(12.0);
                    }
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            // Input-Bereich
            ui.horizontal(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut self.input)
                    .hint_text("Nachricht eingeben...")
                    .desired_width(ui.available_width() - 120.0)
                    .font(egui::TextStyle::Body);
                
                let response = ui.add(text_edit);

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.send_input();
                    response.request_focus();
                }

                if ui.add_sized(
                    [100.0, 30.0],
                    egui::Button::new(
                        egui::RichText::new("Senden")
                            .size(15.0)
                    )
                ).clicked() {
                    self.send_input();
                    response.request_focus();
                }
            });

            // Footer
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Tipps:")
                        .size(13.0)
                        .color(egui::Color32::from_rgb(150, 150, 170))
                );
                ui.label(
                    egui::RichText::new("Wetter • Wikipedia • Programme öffnen")
                        .size(13.0)
                        .color(egui::Color32::from_rgb(120, 120, 140))
                );
            });
            ui.add_space(5.0);
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

pub fn create_window(
    tx: Sender<String>,
    rx: Receiver<String>,
    backend_tx: Sender<String>,
) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 800.0])
            .with_min_inner_size([500.0, 600.0])
            .with_resizable(true)
            .with_title("KI-Assistent"),
        ..Default::default()
    };

    eframe::run_native(
        "KI-Assistent",
        options,
        Box::new(|_cc| Ok(Box::new(AssistantApp::new(tx, rx, backend_tx)))),
    )
}