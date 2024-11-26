use tello_plans::TELLO_PLANS;

pub mod tello_plans;

#[derive(Debug, Clone, Copy)]
enum DataOrMinutesQuantity {
    Limited(usize),
    Unlimited,
}
impl DataOrMinutesQuantity {
    pub fn get_quantity(self) -> usize {
        match self {
            Self::Limited(quantity) => quantity,
            Self::Unlimited => panic!("Cannot get usize of unlimited"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Plan {
    data: DataOrMinutesQuantity,
    minutes: DataOrMinutesQuantity,
    price_per_month: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct DataAndMinutesAmounts {
    data: usize,
    minutes: usize,
}

pub struct Info {
    plans: &'static [Plan],
    current_balance: DataAndMinutesAmounts,
    /// The expected usage every month. Note that if you are planning on needing a lot of data in an upcoming month, you shouldn't use this calculator. You should probably buy 15GB this month.
    expected_usage: DataAndMinutesAmounts,
    reserve_amount: DataAndMinutesAmounts,
}

fn calculate_plan(info: Info) -> Plan {
    let mut plans = info
        .plans
        .iter()
        // For now, filter out the unlimited plans. In the future, or in certain scenarios (using >15GB/mo or >500min/mo), an unlimited plan is the best option. But people who need a ton of minutes or data probably won't be using this calculator anyways.
        .filter(|plan| match (plan.data, plan.minutes) {
            (DataOrMinutesQuantity::Limited(_), DataOrMinutesQuantity::Limited(_)) => true,
            _ => false,
        })
        // Filter out the plans that will not meet the reserve amount
        // Rollover require a limited, >0 amount. If minutes or data are needed, filter out the plans that have 0.
        .filter(|plan| {
            fn add_amount(balance: usize, plan: usize) -> usize {
                if plan > 0 {
                    balance + plan
                } else {
                    0
                }
            }
            let enough_data = add_amount(info.current_balance.data, plan.data.get_quantity())
                > info.reserve_amount.data;
            let enough_minutes =
                add_amount(info.current_balance.minutes, plan.minutes.get_quantity())
                    > info.reserve_amount.minutes;
            enough_data && enough_minutes
        })
        // Filter plans
        .collect::<Vec<_>>();
    println!("Plans: {plans:?}");
    *plans[0]
}

fn main() {
    let best_plan = calculate_plan(Info {
        plans: TELLO_PLANS,
        current_balance: DataAndMinutesAmounts {
            data: 12,
            minutes: 350,
        },
        reserve_amount: DataAndMinutesAmounts {
            data: 8,
            minutes: 300,
        },
        expected_usage: DataAndMinutesAmounts {
            data: 3,
            minutes: 50,
        },
    });
    println!("Best plan: {best_plan:?}");
}
