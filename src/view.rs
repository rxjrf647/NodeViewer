use eframe::{self, egui::{self, RichText, Ui}};
use egui_extras::{Column, TableBuilder};

use crate::data::{self};

fn status_coloer(status: &data::StatCode) -> egui::Color32 {
    match status {
        data::StatCode::Ok => egui::Color32::GREEN,
        data::StatCode::Ng => egui::Color32::RED,
        data::StatCode::Warning => egui::Color32::ORANGE,
    }
}

#[derive(Debug, Clone)]
pub struct MainViewer {
    nodes_info: data::NodesInfo, // for view
    nodes_origin: data::NodesInfo,
    selected: Option<(usize, Option<usize>)>,
    show_alert_only: bool,
}

impl Default for MainViewer {
    fn default() -> Self {
        let nodes_info = data::sample_nodes_info();
        Self {
            nodes_info: nodes_info.clone(),
            nodes_origin: nodes_info,
            selected: None,
            show_alert_only: false
        }
    }
}

impl MainViewer {
    fn reload(&mut self) {
        let data = data::sample_nodes_info();
        self.nodes_info = data.clone();
        self.nodes_origin = data;
        self.selected = None;
        self.show_alert_only = false;
    }
    fn toggle_alert_filter(&mut self) {
        self.show_alert_only = !self.show_alert_only;
        if self.show_alert_only {
            self.nodes_info = data::filter_nodes_alert(&self.nodes_origin);
        } else {
            self.nodes_info = self.nodes_origin.clone();
        }
        self.selected = None;
    }
    fn show_tree(&mut self, ui: &mut Ui) {
        for (i, nodes) in self.nodes_info.iter().enumerate() {
            let tree_id = {
                if self.show_alert_only {
                    RichText::new(format!("{} {:?}     ",nodes.name, nodes.status))
                    .color(status_coloer(&nodes.status))
                } else {
                    RichText::new(format!("{} {:?}  ",nodes.name, nodes.status))
                    .color(status_coloer(&nodes.status))
                }
            };
            let tree_handle: egui::CollapsingResponse<()> = egui::CollapsingHeader::new(tree_id)
            .default_open(self.show_alert_only)
            .show(ui, |ui| {
                for (j, node) in nodes.nodes.iter().enumerate() {
                    let selected = self.selected == Some((i, Some(j)));
                    ui.horizontal(|ui| {
                        if ui.selectable_label(selected, format!("{:?}", node.name)).clicked() {
                            self.selected = Some((i, Some(j)));
                        }
                        ui.label(
                            RichText::new(format!("{:?}", node.status))
                            .color(status_coloer(&node.status))
                        );
                    });
                }
            });
            // select event: Nodes.name
            if tree_handle.header_response.clicked(){
                self.selected = Some((i, None));
            }
            //let selected = self.selected == Some((i, None));
            //if ui.selectable_label(selected, &nodes.name).clicked() {
            //}

        }
    }

    fn show_alert_table(&self, ui: &mut Ui) {
        // ノードROOTを巡回
        for (nodes_id, nodes) in self.nodes_info.iter().enumerate() {
            ui.heading(format!("Nodes: {}", nodes.name));
            ui.push_id(nodes_id, |ui|{
                // ノードを巡回
                for (node_id, node) in nodes.nodes.iter().enumerate() {
                    ui.label(format!("{:?}", node.name));
                    ui.push_id(node_id, |ui| {
                        if !node.contents.is_empty() {
                            TableBuilder::new(ui)
                            .striped(true)
                            .columns(Column::remainder().resizable(true), 3)
                            .header(20.0, |mut header| {
                                header.col(|ui| {ui.label("index");});
                                header.col(|ui| {ui.label("captions");});
                                header.col(|ui| {ui.label("status");});
                            })
                            .body(|mut body| {
                                for content in &node.contents {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(&content.index);
                                        });
                                        row.col(|ui| {
                                            ui.label(&content.caption);
                                        });
                                        row.col(|ui| {
                                            ui.label(
                                                RichText::new(
                                                    format!("{:?}", &content.status)
                                                ).color(status_coloer(&content.status))
                                            );    
                                        });
                                    });
                                }
                            });
                        } else {
                            ui.label("no alert contents");
                        }
                        ui.add_space(5.0);
                    });
                }
                ui.separator();
            });
        }
    }
    
    fn show_table(&mut self, ui: &mut Ui) {
        // セレクト状態情報を入手
        if let Some((nodes_id, node_id)) = self.selected {
            // ノードルートを取得する
            if let Some(nodes) = self.nodes_info.get(nodes_id){
                // ノードノードまで選択されている場合: コンテンツサマリを表示する
                if let Some(node_id) = node_id {
                    // ノードノード情報を参照する（その中のコンテンツがほしい）
                    if let Some(node) = nodes.nodes.get(node_id) {
                        TableBuilder::new(ui)
                        .striped(true)
                        .columns(Column::remainder().resizable(true), 3)
                        .header(20.0, |mut header| {
                            header.col(|ui|{ui.label("index");});
                            header.col(|ui|{ui.label("caption");});
                            header.col(|ui|{ui.label("index");});
                        })
                        .body(|mut body|{
                            for content in &node.contents {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&content.index);
                                    });
                                    row.col(|ui| {
                                        ui.label(&content.caption);
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(format!("{:?}", content.status))
                                            .color(status_coloer(&content.status))
                                        );
                                        //ui.label(format!("{:?}", content.status));
                                    });
                                });
                            }
                        }); 
                    }
                } else {
                // ノードルートが選択されている場合: ノードサマリを表示する
                    TableBuilder::new(ui)
                    //.columns(Column::auto(), 1)
                    .striped(true)
                    .columns(Column::remainder().resizable(true), 2)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label("Node name");
                        });
                        header.col(|ui| {
                            ui.label("Status");
                        });

                    })
                    .body(|mut body| {
                        for node in &nodes.nodes {
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(format!("{:?}", node.name));
                                });
                                row.col(|ui| {
                                    ui.label(
                                        RichText::new(format!("{:?}", node.status))
                                        .color(status_coloer(&node.status))
                                    );
                                });
                            });
                        }
                    });
                }
            }
        }
    }
}

impl eframe::App for MainViewer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header")
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Node Viewer  ");
                if ui.button("filter alerts").clicked() {
                    self.toggle_alert_filter();
                }
                if ui.button("load data").clicked() {
                    self.reload();
                }
            });
            
        });
        egui::SidePanel::left("tree")
        .show(ctx, |ui| {
            ui.label("Node Tree");
            egui::ScrollArea::vertical()
            .show(ui, |ui| {
                self.show_tree(ui);
            });
        });
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
            .show(ui, |ui| {
                //
                if self.show_alert_only {
                    self.show_alert_table(ui);
                } else if let Some((nodes_id, node_id)) = self.selected {
                    ui.label(format!("nodes-id={:?}, node-id={:?}", nodes_id, node_id));
                    self.show_table(ui);
                }
            });
        });
    }
}