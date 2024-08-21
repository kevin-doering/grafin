use clap::Args;

/// The available options for annotation requests
#[derive(Debug, Args, Clone)]
pub struct AnnotationOptions {
    /// Use it in conjunction with the panel_id to add an annotation to a panel of a dashboard
    #[arg(short, long)]
    pub dashboard_uid: Option<String>,
    /// Use it in conjunction with the dashboard_uid to add an annotation to a panel of a dashboard
    #[arg(short, long)]
    pub panel_id: Option<u32>,
    /// Use it to specify the datetime where the annotation should be placed [format: %Y-%m-%d %H:%M]
    #[arg(short, long)]
    pub start_datetime: Option<String>,
    /// Use it to specify a regional annotation with a datetime end [format: %Y-%m-%d %H:%M]
    #[arg(short, long)]
    pub end_datetime: Option<String>,
    /// Use it to add tags to the annotation being added
    #[arg(short, long)]
    pub tags: Vec<String>,
    /// Use it to describe the annotation being added
    #[arg(short, long)]
    pub comment: String,
    /// Use it to add an organizational annotation that is not associated with a panel of a dashboard
    #[arg(short, long, default_value_t = false)]
    pub organizational: bool,
}