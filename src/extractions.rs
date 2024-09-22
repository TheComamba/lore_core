use crate::types::*;

pub fn extract_labels(cols: &[EntityColumn]) -> Vec<Label> {
    let mut labels: Vec<_> = cols.iter().map(|c| c.label.clone()).collect();
    labels.sort();
    labels.dedup();
    labels
}

pub fn extract_descriptors(cols: &[EntityColumn]) -> Vec<Descriptor> {
    let mut descriptors: Vec<_> = cols.iter().map(|c| c.descriptor.clone()).collect();
    descriptors.sort();
    descriptors.dedup();
    descriptors
}

pub fn extract_years(items: &[HistoryItem]) -> Vec<Year> {
    let mut years: Vec<_> = items.iter().map(|item| item.year).collect();
    years.sort();
    years.dedup();
    years
}

pub fn extract_days(items: &[HistoryItem]) -> Vec<Day> {
    let mut days: Vec<_> = items.iter().map(|item| item.day).collect();
    days.sort();
    days.dedup();
    days
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_labels() {
        let cols = vec![
            EntityColumn {
                label: "qux".into(),
                descriptor: "bar".into(),
                description: Description::NONE,
            },
            EntityColumn {
                label: "foo".into(),
                descriptor: "bar".into(),
                description: Description::NONE,
            },
            EntityColumn {
                label: "foo".into(),
                descriptor: "baz".into(),
                description: Description::NONE,
            },
        ];
        let labels = extract_labels(&cols);
        assert_eq!(labels, vec!["foo".into(), "qux".into()]);
    }

    #[test]
    fn test_extract_descriptors() {
        let cols = vec![
            EntityColumn {
                label: "foo".into(),
                descriptor: "bar".into(),
                description: Description::NONE,
            },
            EntityColumn {
                label: "foo".into(),
                descriptor: "baz".into(),
                description: Description::NONE,
            },
            EntityColumn {
                label: "qux".into(),
                descriptor: "bar".into(),
                description: Description::NONE,
            },
        ];
        let descriptors = extract_descriptors(&cols);
        assert_eq!(descriptors, vec!["bar".into(), "baz".into()]);
    }

    #[test]
    fn test_extract_years() {
        use super::*;
        let items = vec![
            HistoryItem {
                timestamp: 0.into(),
                year: 2021.into(),
                day: Day::NONE,
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
            HistoryItem {
                timestamp: 0.into(),
                year: 2020.into(),
                day: Day::NONE,
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
            HistoryItem {
                timestamp: 0.into(),
                year: 2020.into(),
                day: 4.into(),
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
        ];
        let years = extract_years(&items);
        assert!(years == vec![2020.into(), 2021.into()]);
    }

    #[test]
    fn test_extract_days() {
        use super::*;
        let items = vec![
            HistoryItem {
                timestamp: 0.into(),
                year: 2020.into(),
                day: 2.into(),
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
            HistoryItem {
                timestamp: 0.into(),
                year: 2020.into(),
                day: 1.into(),
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
            HistoryItem {
                timestamp: 0.into(),
                year: 2020.into(),
                day: 1.into(),
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
            HistoryItem {
                timestamp: 0.into(),
                year: 2020.into(),
                day: Day::NONE,
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
        ];
        let days = extract_days(&items);
        assert!(days == vec![Day::NONE, 1.into(), 2.into()]);
    }
}
