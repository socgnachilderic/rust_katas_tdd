pub trait TimeProvider {
    fn now(&self) -> u8;
}

pub struct Ohce {
    pub greetings: String,
}

impl Ohce {
    pub fn start(name: &str, time_provider: impl TimeProvider) -> Self {
        let greetings = match time_provider.now() {
            0..=5 => "Buenas noches",
            6..=11 => "Buenos días",
            12..=19 => "Buenas tardes",
            20..=23 => "Buenas noches",
            _ => panic!("time doesn't valid"),
        };

        Self {
            greetings: format!("¡{greetings} {name}!"),
        }
    }

    pub fn echo(&self, input: &str) -> Vec<String> {
        if input == "Stop!" {
            return vec!["Adios Pedro".to_string()];
        }

        let reverse = input.chars().rev().collect::<String>();
        let is_palindrome = reverse == input;
        let mut result = vec![reverse];

        if is_palindrome {
            result.push("¡Bonita palabra!".to_string())
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn should_be_buenas_noches_pedro_when_start_ohce_between_20_and_6_hours() {
        let ohce = Ohce::start("Pedro", NightTimeProvider::default());

        assert_eq!("¡Buenas noches Pedro!", ohce.greetings)
    }

    #[test]
    fn should_be_buenos_dias_pedro_when_start_ohce_between_6_and_12_hours() {
        let ohce = Ohce::start("Pedro", MorningTimeProvider::default());

        assert_eq!("¡Buenos días Pedro!", ohce.greetings)
    }

    #[test]
    fn should_be_buenas_tardes_pedro_when_start_ohce_between_12_and_20_hours() {
        let ohce = Ohce::start("Pedro", EveningTimeProvider::default());

        assert_eq!("¡Buenas tardes Pedro!", ohce.greetings)
    }

    #[test]
    fn should_be_bonita_palabra_when_echo_palindrome_oto() {
        let ohce = Ohce::start("Pedro", MorningTimeProvider::default());

        let response = ohce.echo("oto");

        assert_eq!(vec!["oto", "¡Bonita palabra!"], response)
    }

    #[test]
    fn should_be_aloh_when_echo_hola() {
        let ohce = Ohce::start("Pedro", MorningTimeProvider::default());

        let response = ohce.echo("hola");

        assert_eq!(vec!["aloh"], response)
    }

    #[test]
    fn should_be_pots_when_echo_stop() {
        let ohce = Ohce::start("Pedro", MorningTimeProvider::default());

        let response = ohce.echo("stop");

        assert_eq!(vec!["pots"], response)
    }

    #[test]
    fn should_be_adios_pedro_when_stop() {
        let ohce = Ohce::start("Pedro", MorningTimeProvider::default());

        let response = ohce.echo("Stop!");

        assert_eq!(vec!["Adios Pedro"], response)
    }

    struct NightTimeProvider(u8);

    impl Default for NightTimeProvider {
        fn default() -> Self {
            Self(rand::thread_rng().gen_range(20..24))
        }
    }

    impl TimeProvider for NightTimeProvider {
        fn now(&self) -> u8 {
            self.0
        }
    }

    struct MorningTimeProvider(u8);

    impl Default for MorningTimeProvider {
        fn default() -> Self {
            Self(rand::thread_rng().gen_range(6..12))
        }
    }

    impl TimeProvider for MorningTimeProvider {
        fn now(&self) -> u8 {
            self.0
        }
    }

    struct EveningTimeProvider(u8);

    impl Default for EveningTimeProvider {
        fn default() -> Self {
            Self(rand::thread_rng().gen_range(12..20))
        }
    }

    impl TimeProvider for EveningTimeProvider {
        fn now(&self) -> u8 {
            self.0
        }
    }
}
