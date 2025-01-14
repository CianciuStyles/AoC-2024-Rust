use std::collections::{HashMap, HashSet};

type VertexSet<'a> = HashSet<&'a str>;
type Graph<'a> = HashMap<&'a str, VertexSet<'a>>;

fn parse_input<'a>(text: &'a str) -> Graph<'a> {
    let mut graph = HashMap::new();

    for line in text.lines() {
        let parts: Vec<&'a str> = line.split('-').collect();
        graph
            .entry(parts[0])
            .or_insert(HashSet::new())
            .insert(parts[1]);
        graph
            .entry(parts[1])
            .or_insert(HashSet::new())
            .insert(parts[0]);
    }

    graph
}

fn part1(graph: &Graph) -> u32 {
    let mut result = 0;

    for v1 in graph.keys() {
        for v2 in &graph[v1] {
            for v3 in &graph[v2] {
                if graph[v1].contains(v3)
                    && graph[v2].contains(v3)
                    && v3 > v2
                    && v2 > v1
                    && (v1.starts_with('t') || v2.starts_with('t') || v3.starts_with('t'))
                {
                    result += 1;
                }
            }
        }
    }

    result
}

fn bron_kerbosch<'a>(
    graph: &Graph<'a>,
    r: &mut VertexSet<'a>,
    p: &mut VertexSet<'a>,
    x: &mut VertexSet<'a>,
) -> Vec<VertexSet<'a>> {
    let mut cliques = Vec::new();

    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return cliques;
    }

    let u = p.union(x).next().unwrap();

    for v in p.clone().difference(&graph[u]) {
        let mut new_r: VertexSet = r.iter().copied().collect();
        new_r.insert(v);

        let mut new_p: VertexSet = p.intersection(&graph[v]).copied().collect();
        let mut new_x: VertexSet = x.intersection(&graph[v]).copied().collect();

        cliques.extend(bron_kerbosch(graph, &mut new_r, &mut new_p, &mut new_x));

        p.remove(v);
        x.insert(v);
    }

    cliques
}

fn find_largest_clique<'a>(graph: &Graph<'a>) -> VertexSet<'a> {
    let mut r = HashSet::new();
    let mut p = graph.keys().copied().collect();
    let mut x = HashSet::new();

    let cliques = bron_kerbosch(graph, &mut r, &mut p, &mut x);
    cliques
        .iter()
        .max_by_key(|vertex_set| vertex_set.len())
        .unwrap()
        .clone()
}

fn part2<'a>(graph: &Graph<'a>) -> String {
    let largest_clique = find_largest_clique(graph);

    let mut sorted_keys: Vec<&'a str> = largest_clique.iter().copied().collect();
    sorted_keys.sort_unstable();
    sorted_keys.join(",")
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(&sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 7);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, "co,de,ka,ta");

    let text = helpers::input_file!();
    let input = parse_input(&text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
