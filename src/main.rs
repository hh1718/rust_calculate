mod calculate;
mod svg_graph;
use crate::svg_graph::Graph;

fn main() {
    let sp = calculate::StochasticProcess{ term: 2.0, devide_number: 400 };
    let mut graph = svg_graph::LineGraph{lines: Vec::new()};
    graph.add_line(sp.brownian_motion(), "#223a70");
    graph.add_line(sp.brownian_motion(), "#00a381");
    graph.save("brownian_motion");

    let mut graph2 = svg_graph::LineGraph{lines: Vec::new()};
    graph2.add_line(sp.geometric_brownian_motion(0.01, 0.2, 100.0), "#223a70");
    graph2.add_line(sp.geometric_brownian_motion(0.01, 0.2, 100.0), "#00a381");
    graph2.save("geometric_brownian_motion")
}
