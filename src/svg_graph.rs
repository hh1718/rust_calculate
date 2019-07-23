use plotlib::style::Line;

pub trait Graph {
    fn save(&self, file_name: &str) -> ();
    fn add_line(&mut self, data: Vec<(f64, f64)>, color: &str) -> ();
}

pub struct LineGraph {
    pub lines: Vec<plotlib::line::Line>
}

impl Graph for LineGraph {
    fn save(&self, file_name: &str) -> () {
        let mut v = plotlib::view::ContinuousView::new();
        for line in &self.lines {
            v = v.add(line);
        }
        let fname = file_name.to_string() + &".svg".to_string();
        plotlib::page::Page::single(&v)
            .save(fname)
            .expect("saving svg");
    }

    fn add_line(&mut self, data: Vec<(f64, f64)>, color: &str) -> (){
        let mut lines = vec![plotlib::line::Line::new(&data)
            .style(plotlib::line::Style::new().colour(color))];
        lines.append(&mut self.lines);
        self.lines =  lines;
    }
}