type Node = usize;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Flight {
    pub start: Node,
    pub goal:  Node,
    pub end:   Node,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Solution {
    pub truck_path: Vec<Node>,
    pub flights:    Vec<Flight>,
}

impl Solution {
    pub fn new(truck_path: Vec<Node>, flights: Vec<Flight>) -> Self {
        Self {
            truck_path,
            flights,
        }
    }

    pub fn to_submission_format(&self) -> String {
        let part1 = self.truck_path.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let (flights1, flights2) = self.split_flights()
            .expect("tried to generate submission format of invalid solution");

        let part2_1 = flights1.iter().map(|x| x.goal.to_string());
        let part2_2 = flights2.iter().map(|x| x.goal.to_string());

        let part2 = part2_1.chain(vec!["-1".to_string()].into_iter())
            .chain(part2_2)
            .collect::<Vec<String>>()
            .join(",");

        let index_lookup = self.generate_truck_path_index_lookup();

        let part3_1 = flights1.iter()
            .map(|x| (index_lookup[x.start] + 1).to_string());
        let part3_2 = flights2.iter()
            .map(|x| (index_lookup[x.start] + 1).to_string());

        let part3 = part3_1.chain(vec!["-1".to_string()].into_iter())
            .chain(part3_2)
            .collect::<Vec<String>>()
            .join(",");

        let part4_1 = flights1.iter()
            .map(|x| 
                if x.end == 0 {
                    (*index_lookup.last().unwrap() + 1).to_string()
                } else {
                    (index_lookup[x.end] + 1).to_string()
                }
            );
        let part4_2 = flights2.iter()
            .map(|x| 
                if x.end == 0 {
                    (*index_lookup.last().unwrap() + 1).to_string()
                } else {
                    (index_lookup[x.end] + 1).to_string()
                }
            );


        let part4 = part4_1.chain(vec!["-1".to_string()].into_iter())
            .chain(part4_2)
            .collect::<Vec<String>>()
            .join(",");

        return vec![part1, part2, part3, part4].join("|");
    }

    pub fn generate_truck_path_index_lookup(&self) -> Vec<usize> {
        let lookup_length = self.truck_path.len() + self.flights.len();
        let mut index_lookup = vec![0; lookup_length];

        self.truck_path
            .iter()
            .enumerate()
            .for_each(|(i, &e)| index_lookup[e] = i);

        index_lookup.swap(0, lookup_length - 1);
        return index_lookup;
    }

    pub fn flights_are_feasible(&self) -> bool {
        let index_lookup = self.generate_truck_path_index_lookup();

        for f in self.flights.iter() {
            let start_idx = index_lookup[f.start];
            let end_idx = if f.end == 0 {
                *index_lookup.last().unwrap()
            } else {
                index_lookup[f.end]
            };

            if start_idx >= end_idx {
                return false;
            }

            if f.start == f.goal {
                return false;
            }

            if f.end == f.goal {
                return false;
            }
        }

        return true;
    }

    pub fn sorted_flights(&self) -> Vec<Flight> {
        let index_lookup = self.generate_truck_path_index_lookup();

        let mut flights = self.flights.clone();
        flights.sort_by_key(|x| index_lookup[x.start]);

        return flights;
    }

    pub fn split_flights(&self) -> Result<(Vec<Flight>, Vec<Flight>), &str> {
        // NOTE: should be adapted for more drones
        let mut flights1 = Vec::new();
        let mut flights2 = Vec::new();

        for curr in self.sorted_flights() {
            // NOTE: assumes that flights are valid, i.e. start comes before end
            // if current flight overlaps with previous flight1, push to flights2

            let overlaps_flights1 = flights1.iter().any(|prev|
                self.flights_overlap(&curr, prev));

            let overlaps_flights2 = flights1.iter().any(|prev|
                self.flights_overlap(&curr, prev));

            if !overlaps_flights1 {
                flights1.push(curr.clone());
            } else if !overlaps_flights2 {
                flights2.push(curr.clone());
            } else {
                return Err("cannot split flights without overlap");
            }
        }

        return Ok((flights1, flights2));
    }

    fn flights_overlap(&self, flight1: &Flight, flight2: &Flight) -> bool {
        let index_lookup = self.generate_truck_path_index_lookup();

        let start1 = index_lookup[flight1.start];
        let start2 = index_lookup[flight2.start];

        // NOTE: if node 0 is at the end, then we must take the last element...
        // works for now
        let end1 = if flight1.end == 0 {
            *index_lookup.last().unwrap()
        } else {
            index_lookup[flight1.end]
        };

        let end2 = if flight2.end == 0 {
            *index_lookup.last().unwrap()
        } else {
            index_lookup[flight2.end]
        };

        return start1 < end2 || end1 > start2; 
    }
}
