use easter_egg::{EGraph, Rewrite, Runner, Symbol, SymbolLang, multi_rewrite};

fn main() {
    let mut egraph: EGraph<SymbolLang, ()> = EGraph::default();

    let x = egraph.add_expr(&"x".parse().unwrap());
    let _y = egraph.add_expr(&"y".parse().unwrap());

    let max_x_y = egraph.add_expr(&"(max x y)".parse().unwrap());

    let y_le_x_color = egraph.create_color(None);

    // max(x, y) = x if y <= x
    let simp_max: Rewrite<SymbolLang, ()> = multi_rewrite!(
        "simp_max";
        "?lhs = (max ?x ?y), ?cond = (istrue (le ?y ?x)), ?cond = TRUE" => "?lhs = ?x"
    );

    let rules = vec![
        simp_max
    ];


    let true_id= egraph.add_expr(&"TRUE".parse().unwrap());
    let cond_id = egraph.add_expr(&"(istrue (le y x))".parse().unwrap());
    egraph.colored_union(y_le_x_color, cond_id, true_id);

    assert_eq!(egraph.colored_find(y_le_x_color, cond_id), egraph.colored_find(y_le_x_color, true_id));

    let runner: Runner<SymbolLang, ()> = Runner::new(())
        .with_egraph(egraph)
        .run(&rules);

    let result_egraph = runner.egraph;
    assert!(result_egraph.find(max_x_y) != result_egraph.find(x));
    assert_eq!(result_egraph.colored_find(y_le_x_color, max_x_y), result_egraph.colored_find(y_le_x_color, x));

}
