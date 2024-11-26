use crate::{DataOrMinutesQuantity, Plan};

pub const TELLO_PLANS: &[Plan] = &[
    Plan {
        data: DataOrMinutesQuantity::Limited(0),
        minutes: DataOrMinutesQuantity::Limited(100),
        price_per_month: 5,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(0),
        minutes: DataOrMinutesQuantity::Limited(300),
        price_per_month: 6,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(0),
        minutes: DataOrMinutesQuantity::Limited(500),
        price_per_month: 7,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(0),
        minutes: DataOrMinutesQuantity::Unlimited,
        price_per_month: 8,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(1),
        minutes: DataOrMinutesQuantity::Limited(0),
        price_per_month: 5,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(1),
        minutes: DataOrMinutesQuantity::Limited(100),
        price_per_month: 6,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(1),
        minutes: DataOrMinutesQuantity::Limited(300),
        price_per_month: 7,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(1),
        minutes: DataOrMinutesQuantity::Limited(500),
        price_per_month: 8,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(1),
        minutes: DataOrMinutesQuantity::Unlimited,
        price_per_month: 9,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(2),
        minutes: DataOrMinutesQuantity::Limited(0),
        price_per_month: 6,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(2),
        minutes: DataOrMinutesQuantity::Limited(100),
        price_per_month: 7,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(2),
        minutes: DataOrMinutesQuantity::Limited(300),
        price_per_month: 8,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(2),
        minutes: DataOrMinutesQuantity::Limited(500),
        price_per_month: 9,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(2),
        minutes: DataOrMinutesQuantity::Unlimited,
        price_per_month: 10,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(5),
        minutes: DataOrMinutesQuantity::Limited(0),
        price_per_month: 10,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(5),
        minutes: DataOrMinutesQuantity::Limited(100),
        price_per_month: 11,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(5),
        minutes: DataOrMinutesQuantity::Limited(300),
        price_per_month: 12,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(5),
        minutes: DataOrMinutesQuantity::Limited(500),
        price_per_month: 13,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(5),
        minutes: DataOrMinutesQuantity::Unlimited,
        price_per_month: 14,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(5),
        minutes: DataOrMinutesQuantity::Unlimited,
        price_per_month: 14,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(10),
        minutes: DataOrMinutesQuantity::Limited(0),
        price_per_month: 15,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(10),
        minutes: DataOrMinutesQuantity::Limited(100),
        price_per_month: 16,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(10),
        minutes: DataOrMinutesQuantity::Limited(300),
        price_per_month: 17,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(10),
        minutes: DataOrMinutesQuantity::Limited(500),
        price_per_month: 18,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(10),
        minutes: DataOrMinutesQuantity::Unlimited,
        price_per_month: 19,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(15),
        minutes: DataOrMinutesQuantity::Limited(0),
        price_per_month: 20,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(15),
        minutes: DataOrMinutesQuantity::Limited(100),
        price_per_month: 21,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(15),
        minutes: DataOrMinutesQuantity::Limited(300),
        price_per_month: 22,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(15),
        minutes: DataOrMinutesQuantity::Limited(500),
        price_per_month: 23,
    },
    Plan {
        data: DataOrMinutesQuantity::Limited(15),
        minutes: DataOrMinutesQuantity::Unlimited,
        price_per_month: 24,
    },
    // Ignore the other minutes options since they cost $25/mo too
    Plan {
        data: DataOrMinutesQuantity::Unlimited,
        minutes: DataOrMinutesQuantity::Unlimited,
        price_per_month: 25,
    },
];
