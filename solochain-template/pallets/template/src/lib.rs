#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        type WeightInfo: WeightInfo;

        #[pallet::constant]
        type MaxNomeLength: Get<u32>;
        #[pallet::constant]
        type MaxStatusLength: Get<u32>;
    }

    #[pallet::storage]
    pub type Alunos<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        (BoundedVec<u8, T::MaxNomeLength>, BoundedVec<u8, T::MaxStatusLength>)
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AlunoRegistrado {
            id: T::AccountId,
            nome: BoundedVec<u8, T::MaxNomeLength>,
            status: BoundedVec<u8, T::MaxStatusLength>,
        },
        AlunoAtualizado {
            id: T::AccountId,
            nome: BoundedVec<u8, T::MaxNomeLength>,
            status: BoundedVec<u8, T::MaxStatusLength>,
        },
        AlunoDeletado {
            id: T::AccountId,
        },
        AlunosListados {
            count: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NomeInvalido,
        StatusInvalido,
        AlunoJaExiste,
        AlunoNaoEncontrado,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn criar_aluno(
            origin: OriginFor<T>,
            nome: Vec<u8>,
            status: Vec<u8>
        ) -> DispatchResult {
            let id = ensure_signed(origin)?;

            ensure!(!Alunos::<T>::contains_key(&id), Error::<T>::AlunoJaExiste);

            let nome_bounded: BoundedVec<_, T::MaxNomeLength> = nome
                .try_into()
                .map_err(|_| Error::<T>::NomeInvalido)?;

            let status_bounded: BoundedVec<_, T::MaxStatusLength> = status
                .try_into()
                .map_err(|_| Error::<T>::StatusInvalido)?;

            Alunos::<T>::insert(&id, (nome_bounded.clone(), status_bounded.clone()));

            Self::deposit_event(Event::AlunoRegistrado {
                id,
                nome: nome_bounded,
                status: status_bounded,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        pub fn atualizar_aluno(
            origin: OriginFor<T>,
            nome: Vec<u8>,
            status: Vec<u8>
        ) -> DispatchResult {
            let id = ensure_signed(origin)?;

            ensure!(Alunos::<T>::contains_key(&id), Error::<T>::AlunoNaoEncontrado);

            let nome_bounded: BoundedVec<_, T::MaxNomeLength> = nome
                .try_into()
                .map_err(|_| Error::<T>::NomeInvalido)?;

            let status_bounded: BoundedVec<_, T::MaxStatusLength> = status
                .try_into()
                .map_err(|_| Error::<T>::StatusInvalido)?;

            Alunos::<T>::insert(&id, (nome_bounded.clone(), status_bounded.clone()));

            Self::deposit_event(Event::AlunoAtualizado {
                id,
                nome: nome_bounded,
                status: status_bounded,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::default())]
        pub fn deletar_aluno(origin: OriginFor<T>) -> DispatchResult {
            let id = ensure_signed(origin)?;

            ensure!(Alunos::<T>::contains_key(&id), Error::<T>::AlunoNaoEncontrado);

            Alunos::<T>::remove(&id);

            Self::deposit_event(Event::AlunoDeletado { id });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::default())]
        pub fn listar_alunos(origin: OriginFor<T>) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            let count = Alunos::<T>::iter().count() as u32;

            Self::deposit_event(Event::AlunosListados { count });

            for (id, (nome, status)) in Alunos::<T>::iter() {
                Self::deposit_event(Event::AlunoRegistrado {
                    id,
                    nome: nome.clone(),
                    status: status.clone(),
                });
            }

            Ok(())
        }
    }
}
