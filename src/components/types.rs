#[derive(Clone, PartialEq)]
pub enum ActiveView {
    Viewer,
    Traces,
}

#[derive(Clone, PartialEq)]
pub enum Channel {
    PhaseContrast,
    Fluorescence,
    Segmentation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternState {
    pub has_cell: bool,
    pub status: Option<PatternStatus>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatternStatus {
    Confirmed,
    Rejected,
}
