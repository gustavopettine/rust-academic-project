#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[ink::contract]
mod aluno_contract {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::prelude::format;

    #[ink(storage)]
    pub struct AlunoContract {
        alunos: Vec<String>,
    }

    impl AlunoContract {
        #[ink(constructor)]
        pub fn new(alunos_iniciais: Vec<String>) -> Self {
            Self {
                alunos: alunos_iniciais,
            }
        }

        #[ink(message)]
        pub fn registra_aluno(&mut self, id: AccountId, nome: String, status: String) {
            let aluno = format!("{:?}|{}|{}", id, nome, status);
            self.alunos.push(aluno);
        }

        #[ink(message)]
        pub fn lista_alunos(&self) -> Vec<String> {
            self.alunos.clone()
        }

        #[ink(message)]
        pub fn encontra_aluno(&self, id: AccountId) -> Option<String> {
            let id_str = format!("{:?}", id);
            self.alunos.iter().find(|&aluno| aluno.starts_with(&id_str)).cloned()
        }

        #[ink(message)]
        pub fn atualiza_aluno(&mut self, id: AccountId, nome: String, status: String) {
            let id_str = format!("{:?}", id);
            if let Some(index) = self.alunos.iter().position(|aluno| aluno.starts_with(&id_str)) {
                let aluno = format!("{:?}|{}|{}", id, nome, status);
                self.alunos[index] = aluno;
            }
        }

        #[ink(message)]
        pub fn deleta_aluno(&mut self, id: AccountId) {
            let id_str = format!("{:?}", id);
            self.alunos.retain(|aluno| !aluno.starts_with(&id_str));
        }
    }
}
