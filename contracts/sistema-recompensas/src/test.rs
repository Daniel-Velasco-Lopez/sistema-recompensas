#![cfg(test)]

use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

use crate::{Contract, ContractClient};

#[test]
fn test_registrar_y_consultar_estudiante() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let estudiante = Address::random(&env);
    let nombre = symbol_short!("Daniel");

    client.registrar_estudiante(&estudiante, &nombre);
    let puntos = client.consultar_puntos(&estudiante);
    assert_eq!(puntos, 0);
}

#[test]
fn test_acumular_puntos() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let estudiante = Address::random(&env);
    let nombre = symbol_short!("Maria");
    client.registrar_estudiante(&estudiante, &nombre);

    client.acumular_puntos(&estudiante, &symbol_short!("quiz1"), &10);
    client.acumular_puntos(&estudiante, &symbol_short!("quiz2"), &5);

    let puntos = client.consultar_puntos(&estudiante);
    assert_eq!(puntos, 15);

    let historial = client.consultar_historial(&estudiante);
    assert_eq!(historial.len(), 2);
}

#[test]
fn test_recompensas() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let estudiante = Address::random(&env);
    let nombre = symbol_short!("Luis");
    client.registrar_estudiante(&estudiante, &nombre);

    client.acumular_puntos(&estudiante, &symbol_short!("examen"), &50);

    let ok = client.solicitar_recompensa(&estudiante, &symbol_short!("Sticker"), &30);
    assert!(ok);

    let puntos_restantes = client.consultar_puntos(&estudiante);
    assert_eq!(puntos_restantes, 20);

    let recompensas = client.consultar_recompensas(&estudiante);
    assert_eq!(recompensas.len(), 1);
}
