/*
 * Copyright 2021 Julian Schmidhuber <github@schmiddi.anonaddy.com>
 *
 * This file is part of Tubefeeder.
 *
 * Tubefeeder is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Tubefeeder is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Tubefeeder.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::sync::Arc;
use std::sync::Mutex;

use crate::gui::{get_font_size, FONT_RATIO};

use gtk::prelude::*;
use gtk::Align;
use gtk::Orientation::Vertical;
use pango::{AttrList, Attribute, EllipsizeMode};
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};
use tf_filter::FilterGroup;
use tf_join::AnyVideoFilter;

#[derive(Msg)]
pub enum FilterItemMsg {
    Remove,
}

pub struct FilterItemModel {
    filter: AnyVideoFilter,
    _filters: Arc<Mutex<FilterGroup<AnyVideoFilter>>>,
}

#[widget]
impl Widget for FilterItem {
    fn model(
        _: &Relm<Self>,
        (filter, _filters): (AnyVideoFilter, Arc<Mutex<FilterGroup<AnyVideoFilter>>>),
    ) -> FilterItemModel {
        FilterItemModel { filter, _filters }
    }

    fn update(&mut self, event: FilterItemMsg) {
        match event {
            FilterItemMsg::Remove => {
                todo!()
            }
        }
    }

    fn init_view(&mut self) {
        let font_size = get_font_size();
        let title_attr_list = AttrList::new();
        title_attr_list.insert(Attribute::new_size(font_size * pango::SCALE));
        self.widgets
            .label_title
            .set_attributes(Some(&title_attr_list));

        let channel_attr_list = AttrList::new();
        channel_attr_list.insert(Attribute::new_size(
            (FONT_RATIO * (font_size * pango::SCALE) as f32) as i32,
        ));
        self.widgets
            .label_channel
            .set_attributes(Some(&channel_attr_list));
    }

    view! {
        gtk::ListBoxRow {
            gtk::Box {
                gtk::Button {
                    image: Some(&gtk::Image::from_icon_name(Some("list-remove-symbolic"), gtk::IconSize::LargeToolbar)),
                    clicked => FilterItemMsg::Remove,
                },
                gtk::Box {
                    orientation: Vertical,
                    #[name="label_title"]
                    gtk::Label {
                        text: &self.model.filter.title_str().unwrap_or("".to_string()),
                        ellipsize: EllipsizeMode::End,
                        halign: Align::Start
                    },
                    #[name="label_channel"]
                    gtk::Label {
                        text: &self.model.filter.subscription_str().unwrap_or("".to_string()),
                        ellipsize: EllipsizeMode::End,
                        halign: Align::Start
                    },
                }
            }
        }
    }
}
