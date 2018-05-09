use std::collections::HashMap;
use std::collections::HashSet;

type PID = u32;
type TID = u32;
type Table = HashSet<PID>;


struct AssociationManager {
    people_to_table: HashMap<PID, TID>,
    table_members: HashMap<TID, Table>,
}


impl AssociationManager {
    fn new(n_people: u32, n_tables: u32, table_size: usize) -> AssociationManager {
        let people_to_table = HashMap::with_capacity(n_people as usize);

        let mut table_members = HashMap::new();
        for i in 0..n_tables {
            table_members.insert(i, HashSet::with_capacity(table_size));
        }
        AssociationManager{
            people_to_table: people_to_table,
            table_members: table_members,
        }
    }

    fn join_table(&mut self, pid: &PID, tid: &TID) -> Option<TID> {
        if let Some(members) = self.table_members.get_mut(tid) {
            members.insert(*pid);
        }

        let maybe_tid = self.people_to_table.insert(*pid, *tid);
        maybe_tid
    }

    fn curr_table(&self, pid: &PID) -> Option<&Table> {
        match self.people_to_table.get(pid) {
            Some(tid) => { Some(self.table_members.get(tid).unwrap()) },
            None => { None }
        }
    }
}


#[allow(dead_code)]
struct TableSeq {
    tables: Vec<TID>,
    index: usize,
    _next: Option<TID>,
}


impl TableSeq {
    fn new(tables: &Vec<TID>) -> TableSeq {
        TableSeq {
            tables: tables.clone(),
            index: tables.len() - 1,
            _next: None,
        }
    }

    fn prepend(&mut self, next_item: TID) {
        self._next = Some(next_item);
    }
}

impl Iterator for TableSeq {
    type Item = TID;
    fn next(&mut self) -> Option<TID> {
        if let Some(_next) = self._next {
            self._next = None;
            Some(_next)
        } else {
            self.index = (self.index + 1) % self.tables.len();
            Some(*self.tables.get(self.index).unwrap())
        }
    }
}


enum TableAction {
    Join(TID),
    Replace(PID, TID),
    Stay,
}


fn decide_action(pid: PID, curr_table: &Table, table_seq: &mut TableSeq) -> (bool, TableAction) {
    let base = score_join(pid, curr_table);

    let mut best = u32::MAX;
    let mut best_action = TableAction.Stay;

    let table_seq.one_pass()
        .map(|tid, table| (score_join(pid, table), tid))
        .filter(|table, tid| Some(table))
        .
        ;

    // loop over all tables once
    for (tid, table) in table_seq.one_pass() {
        // if the table is happier with this person, join the table
        if let Some(s) = score_join(pid, table) {
            best = s;
            best_action = TableAction.Join(tid);
        }

        // if the table is happier 
        for (kicked_pid, left) in drop_one(table) {
            if let Some(s) = score_join(pid, left) {
                if s < best {
                    best = s;
                    best_action = TableAction.Join(tid);
                }
            }
        }
    }
    return (best < base, best_action)
}


fn score_join(pid: PID, members: &Table) -> Option<u32> {
    let max_size = 2;
    let mut members = members.clone();
    members.insert(pid);
    if members.len() > max_size {
        return None
    }
    return Some(members.len() as u32);
}

fn main() {
    println!("printing");

    let n_tables = 3;
    let n_people = 6;
    let mut man = AssociationManager::new(n_people, n_tables, 2);

    let mut table_seq = TableSeq::new(&(0..3).collect());

    let peeps = (0..n_people).chain(0..n_people);
    for pid in peeps {
//        let old_tid = man.join_table(&pid, &new_tid);
//        print!("{} old -> new {:?}\n", pid, old_tid);
    }

    for (k, v) in &man.people_to_table {
        println!("{}, {}", k, v);
    }
}
