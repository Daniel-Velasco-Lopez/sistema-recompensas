#![no_std]

use soroban_sdk::{contractimpl, contracttype, Env, Symbol, Address, Vec};

#[derive(Clone)]
#[contracttype]
pub struct Estudiante {
    pub nombre: Symbol,
    pub puntos: i32,
    pub historial: Vec<Symbol>,
    pub recompensas: Vec<Symbol>,
}

pub struct Contract; // Primero declaramos el struct vacío

#[contractimpl]
impl Contract {
    pub fn registrar_estudiante(env: Env, id: Address, nombre: Symbol) {
        if env.storage().persistent().has(&id) {
            panic!("El estudiante ya está registrado");
        }

        let estudiante = Estudiante {
            nombre,
            puntos: 0,
            historial: Vec::new(&env),
            recompensas: Vec::new(&env),
        };

        env.storage().persistent().set(&id, &estudiante);
    }

    pub fn acumular_puntos(env: Env, id: Address, actividad_id: Symbol, puntos: i32) {
        let mut estudiante: Estudiante = env
            .storage()
            .persistent()
            .get(&id)
            .expect("Estudiante no encontrado");

        estudiante.puntos += puntos;
        estudiante.historial.push_back(actividad_id);

        env.storage().persistent().set(&id, &estudiante);
    }

    pub fn consultar_historial(env: Env, id: Address) -> Vec<Symbol> {
        let estudiante: Estudiante = env
            .storage()
            .persistent()
            .get(&id)
            .expect("Estudiante no encontrado");

        estudiante.historial
    }

    pub fn consultar_puntos(env: Env, id: Address) -> i32 {
        let estudiante: Estudiante = env
            .storage()
            .persistent()
            .get(&id)
            .expect("Estudiante no encontrado");

        estudiante.puntos
    }

    pub fn solicitar_recompensa(env: Env, id: Address, recompensa_id: Symbol, costo: i32) -> bool {
        let mut estudiante: Estudiante = env
            .storage()
            .persistent()
            .get(&id)
            .expect("Estudiante no encontrado");

        if estudiante.puntos < costo {
            return false;
        }

        estudiante.puntos -= costo;
        estudiante.recompensas.push_back(recompensa_id);

        env.storage().persistent().set(&id, &estudiante);
        true
    }

    pub fn consultar_recompensas(env: Env, id: Address) -> Vec<Symbol> {
        let estudiante: Estudiante = env
            .storage()
            .persistent()
            .get(&id)
            .expect("Estudiante no encontrado");

        estudiante.recompensas
    }
}
 