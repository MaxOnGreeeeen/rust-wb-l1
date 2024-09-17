fn main() {
    let subscriber_user = SubscriberContextProvider {
        subscriber: Subscriber {
            name: "Test testov".to_owned(),
            location: "Moscow".to_owned(),
        },
    };
    request_full_info_user(subscriber_user);

    let operator_context = OperatorContextProvider {
        operator: Operator {
            dep_name: "Operator Operatorov".to_owned(),
            location: "Moscow".to_owned(),
        },
    };
    let operator_context_provider = OperatorContextProviderAdapter::new(operator_context);
    request_full_info_user(operator_context_provider);
}

pub fn request_full_info_user(user: impl ContextProvider) {
    println!("Full user info: {}", user.get_info());
}

// Типаж для получения информации о пользователе
pub trait ContextProvider {
    fn get_info(&self) -> String;
}

pub struct Operator {
    dep_name: String,
    location: String,
}

// Кастомный контекст оператора с методом, отличающимся от get_info()
pub struct OperatorContextProvider {
    operator: Operator,
}
impl OperatorContextProvider {
    pub fn get_operator_context_info(&self) -> String {
        return self.get_operator_info();
    }

    fn get_operator_info(&self) -> String {
        format!(
            "Location: {}, Dep. name {}",
            self.operator.location, self.operator.dep_name
        )
    }
}
// Адаптер ContextProvider для OperatorContextProvider
pub struct OperatorContextProviderAdapter {
    adaptee: OperatorContextProvider,
}
impl OperatorContextProviderAdapter {
    pub fn new(adaptee: OperatorContextProvider) -> Self {
        Self { adaptee }
    }
}
impl ContextProvider for OperatorContextProviderAdapter {
    fn get_info(&self) -> String {
        self.adaptee.get_operator_context_info()
    }
}

pub struct Subscriber {
    name: String,
    location: String,
}

// Контекст пользователя с реализованным типажом ContextProvider
pub struct SubscriberContextProvider {
    subscriber: Subscriber,
}
impl ContextProvider for SubscriberContextProvider {
    fn get_info(&self) -> String {
        format!(
            "Subscriber name: {}, Location {}",
            self.subscriber.name, self.subscriber.location
        )
    }
}
