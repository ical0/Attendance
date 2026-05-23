#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, Symbol, String,
};

#[contracttype]
#[derive(Clone)]
pub struct Attendance {
    pub name: String,
    pub hadir: bool,
}

const ATTENDANCE: Symbol = symbol_short!("ATTEND");

#[contract]
pub struct AttendanceContract;

#[contractimpl]
impl AttendanceContract {

    // Tambah data hadir
    pub fn add_attendance(env: Env, name: String) {

        let data = Attendance {
            name: name.clone(),
            hadir: true,
        };

        env.storage().persistent().set(&name, &data);

        // counter total hadir
        let mut total: u32 =
            env.storage()
               .persistent()
               .get(&ATTENDANCE)
               .unwrap_or(0);

        total += 1;

        env.storage()
            .persistent()
            .set(&ATTENDANCE, &total);
    }

    // cek hadir
    pub fn check_attendance(
        env: Env,
        name: String
    ) -> bool {

        let result: Option<Attendance> =
            env.storage()
               .persistent()
               .get(&name);

        match result {
            Some(data) => data.hadir,
            None => false,
        }
    }

    // total hadir
    pub fn total_attendance(env: Env) -> u32 {

        env.storage()
            .persistent()
            .get(&ATTENDANCE)
            .unwrap_or(0)
    }
}