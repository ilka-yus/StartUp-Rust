use std::fmt;

// 1. Константа для конвертации (1 SOL = 1,000,000,000 Lamports)
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// 2. Структура аккаунта (struct)
#[derive(Debug)]
struct UserAccount {
    pubkey: String,
    lamports: u64,
}

impl UserAccount {
    // Конвертация Lamports -> SOL (возвращает f64)
    fn get_sol_balance(&self) -> f64 {
        self.lamports as f64 / LAMPORTS_PER_SOL as f64
    }

    // Метод с использованием Option для безопасного снятия средств
    fn withdraw(&mut self, amount_lamports: u64) -> Option<u64> {
        if self.lamports >= amount_lamports {
            self.lamports -= amount_lamports;
            Some(self.lamports)
        } else {
            None // Недостаточно баланса
        }
    }
}

// 3. Перечисление статусов транзакции (enum)
#[derive(Debug)]
enum TransactionStatus {
    Success,
    Failed(String),
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionStatus::Success => write!(f, "Успешно [SUCCESS]"),
            TransactionStatus::Failed(reason) => write!(f, "Ошибка [FAILED]: {}", reason),
        }
    }
}

// 4. Проверка валидности публичного ключа (Result & Option)
fn validate_pubkey(pubkey: &str) -> Result<&str, String> {
    if pubkey.is_empty() {
        return Err("Публичный ключ не может быть пустым!".to_string());
    }
    // В Solana Base58 публичные ключи обычно имеют длину от 32 до 44 символов
    if pubkey.len() < 32 || pubkey.len() > 44 {
        return Err(format!(
            "Неверная длина ключа ({} символов). Ожидается от 32 до 44.",
            pubkey.len()
        ));
    }
    Ok(pubkey)
}

// Функция-конвертер SOL в Lamports
fn sol_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_SOL as f64) as u64
}

fn main() {
    println!("==================================================");
    println!("        SOLANA CLI PRACTICE - WEEK 3 (RUST)      ");
    println!("==================================================\n");

    // --- Демонстрация 1: Валидация адреса (Result & match) ---
    println!("[1] Проверка валидации публичных ключей (Result & match):");
    let valid_key = "3g8BRdEkLVJjB2SLAzj7BoBDs7zQUJzFRzjh9Cmvgz3P";
    let invalid_key = "short_key_123";

    for key in [valid_key, invalid_key] {
        match validate_pubkey(key) {
            Ok(k) => println!("  ✔ Ключ '{}' валиден!", k),
            Err(e) => println!("  ✖ Ошибка проверки ключа '{}': {}", key, e),
        }
    }
    println!();

    // --- Демонстрация 2: Создание аккаунта и конвертация Lamports/SOL ---
    println!("[2] Работа со структурой UserAccount и конвертация юнитов:");
    let initial_lamports = sol_to_lamports(2.5); // 2.5 SOL в lamports
    let mut account = UserAccount {
        pubkey: valid_key.to_string(),
        lamports: initial_lamports,
    };

    println!("  Аккаунт создан: {:?}", account.pubkey);
    println!("  Баланс в Lamports: {} lamports", account.lamports);
    println!("  Баланс в SOL: {:.4} SOL", account.get_sol_balance());
    println!();

    // --- Демонстрация 3: Безопасное снятие средств (Option) ---
    println!("[3] Симуляция списания средств (Option & match):");
    let withdraw_amount = sol_to_lamports(1.0);
    println!("  Попытка списать 1.0 SOL ({} lamports)...", withdraw_amount);

    match account.withdraw(withdraw_amount) {
        Some(new_balance) => {
            println!("  ✔ Списание прошло успешно!");
            println!("  Новый баланс: {} lamports ({:.4} SOL)", new_balance, account.get_sol_balance());
        }
        None => println!("  ✖ Ошибка: недостаточный баланс!"),
    }
    println!();

    // --- Демонстрация 4: Статусы транзакций (Enum) ---
    println!("[4] Моделирование статусов транзакций (Enum):");
    let tx1 = TransactionStatus::Success;
    let tx2 = TransactionStatus::Failed("Custom error: Insufficient funds for fee".to_string());

    println!("  Транзакция 1: {}", tx1);
    println!("  Транзакция 2: {}", tx2);
    println!("\n==================================================");
}