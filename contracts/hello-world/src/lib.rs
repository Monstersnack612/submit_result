#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contract]
pub struct CanteenCredit;

#[contractimpl]
impl CanteenCredit {
    /// HÀM INIT MỚI: Thiết lập Admin VÀ nạp tiền sẵn cho một sinh viên ngay từ đầu.
    pub fn init(env: Env, admin: Address, default_student: Address, initial_amount: u32) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract đã được khởi tạo!");
        }
        
        // 1. Lưu địa chỉ Admin
        env.storage().instance().set(&DataKey::Admin, &admin);
        
        // 2. Nạp tiền luôn cho sinh viên để test cho lẹ, khỏi gọi hàm recharge
        env.storage().persistent().set(&DataKey::Balance(default_student), &initial_amount);
    }

    /// Admin nạp thêm tín dụng (Vẫn giữ lại phòng khi bạn muốn nạp thêm)
    pub fn recharge(env: Env, admin: Address, student: Address, amount: u32) {
        admin.require_auth(); 
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("Chỉ Admin mới có quyền nạp tín dụng!");
        }

        let mut balance = Self::get_balance(env.clone(), student.clone());
        balance += amount;
        env.storage().persistent().set(&DataKey::Balance(student), &balance);
    }

    /// Sinh viên sử dụng token để thanh toán
    pub fn pay_meal(env: Env, student: Address, amount: u32) {
        student.require_auth(); 

        let mut balance = Self::get_balance(env.clone(), student.clone());
        if balance < amount {
            panic!("Số dư tín dụng không đủ!");
        }

        balance -= amount;
        env.storage().persistent().set(&DataKey::Balance(student.clone()), &balance);
    }

    /// Xem số dư
    pub fn get_balance(env: Env, student: Address) -> u32 {
        env.storage().persistent().get(&DataKey::Balance(student)).unwrap_or(0)
    }
}