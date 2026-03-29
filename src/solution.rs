type Node = usize;

#[derive(Clone, Debug, Default)]
pub struct Flight {
    pub start: Node,
    pub goal:  Node,
    pub end:   Node,
}

#[derive(Clone, Debug, Default)]
pub struct Solution {
    pub truck_path: Vec<Node>,
    pub flights:    Vec<Flight>,
}

impl Solution {
    pub fn to_submission_format(&self) -> String {
        let part1 = self.truck_path.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let (flights1, flights2) = self.split_flights();

        let part2_1 = flights1.iter().map(|x| x.goal.to_string());
        let part2_2 = flights2.iter().map(|x| x.goal.to_string());

        let part2 = part2_1.chain(vec!["-1".to_string()].into_iter())
            .chain(part2_2)
            .collect::<Vec<String>>()
            .join(",");

        let index_lookup = self.generate_truck_path_index_lookup();

        // NOTE: have to differentiate between 'first' and 'last' depot

        let part3_1 = flights1.iter()
            .map(|x| (index_lookup[x.start] + 1).to_string());
        let part3_2 = flights2.iter()
            .map(|x| (index_lookup[x.start] + 1).to_string());

        let part3 = part3_1.chain(vec!["-1".to_string()].into_iter())
            .chain(part3_2)
            .collect::<Vec<String>>()
            .join(",");

        let part4_1 = flights1.iter()
            .map(|x| (index_lookup[x.end] + 1).to_string());
        let part4_2 = flights2.iter()
            .map(|x| (index_lookup[x.end] + 1).to_string());

        let part4 = part4_1.chain(vec!["-1".to_string()].into_iter())
            .chain(part4_2)
            .collect::<Vec<String>>()
            .join(",");

        return vec![part1, part2, part3, part4].join("|");
    }

    fn generate_truck_path_index_lookup(&self) -> Vec<usize> {
        let lookup_length = self.truck_path.len() + self.flights.len();

        let mut truck_path_index_lookup = vec![0; lookup_length];
        for (i, &e) in self.truck_path.iter().enumerate() {
            truck_path_index_lookup[e] = i;
        }
        // swap so that the first element is the index of the first depot and 
        // the last is the index of the last depot
        truck_path_index_lookup.swap(0, lookup_length - 1);

        return truck_path_index_lookup;
    }

    pub fn flights_deploy_in_order(&self) -> bool {
        let index_lookup = self.generate_truck_path_index_lookup();

        let start_idxs: Vec<usize> = self.flights.iter()
            .map(|f| index_lookup[f.start])
            .collect();

        return start_idxs.windows(2).all(|w| w[0] <= w[1]);
    }

    fn split_flights(&self) -> (Vec<Flight>, Vec<Flight>) {
        // NOTE: should be adapted for more drones
        assert!(self.flights_deploy_in_order());

        let mut flights1 = Vec::new();
        let mut flights2 = Vec::new();

        for curr in self.flights.iter() {
            // NOTE: assumes that flights are valid, i.e. start comes before end
            // if current flight overlaps with previous flight1, push to flights2

            if let Some(prev) = flights1.last() {
                if self.flights_overlap(curr, prev) {
                    flights2.push(curr.clone());
                } else {
                    flights1.push(curr.clone());
                }
            } else {
                flights1.push(curr.clone());
            }
        }

        return (flights1, flights2);
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
            index_lookup[flight1.end]
        };

        return start1 >= end2 || end1 <= start2; 
    }
}
