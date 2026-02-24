use crate::Item;

pub(in crate::search) fn format_item_name_list_comma_separated(items: &[Item]) -> String {
    items
        .iter()
        .map(|item| item.name.clone())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
#[path = "tests/item_name_list_formatting_tests.rs"]
mod item_name_list_formatting_tests;
