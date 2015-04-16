use columnvalueops::{ColumnValueOps, ColumnValueOpsExt};
use super::super::sexpression::AggregateOp;

pub trait AggregateFunction<ColumnValue> {
    fn feed(&mut self, value: ColumnValue);
    fn finish(self: Box<Self>) -> ColumnValue;
}

struct Count {
    count: u64
}

impl<ColumnValue: ColumnValueOps> AggregateFunction<ColumnValue> for Count
{
    fn feed(&mut self, value: ColumnValue) {
        if !value.is_null() {
            self.count += 1;
        }
    }

    fn finish(self: Box<Self>) -> ColumnValue {
        ColumnValueOps::from_u64(self.count)
    }
}

struct Avg {
    sum: f64,
    count: u64
}

impl<ColumnValue: ColumnValueOps> AggregateFunction<ColumnValue> for Avg {
    fn feed(&mut self, value: ColumnValue) {
        if !value.is_null() {
            self.sum += value.to_f64().unwrap();
            self.count += 1;
        }
    }

    fn finish(self: Box<Self>) -> ColumnValue {
        if self.count == 0 {
            ColumnValueOpsExt::null()
        } else {
            ColumnValueOps::from_f64(self.sum / (self.count as f64))
        }
    }
}

struct Sum {
    sum: f64,
    count: u64
}

impl<ColumnValue: ColumnValueOps> AggregateFunction<ColumnValue> for Sum {
    fn feed(&mut self, value: ColumnValue) {
        if !value.is_null() {
            self.sum += value.to_f64().unwrap();
            self.count += 1;
        }
    }

    fn finish(self: Box<Self>) -> ColumnValue {
        if self.count == 0 {
            ColumnValueOpsExt::null()
        } else {
            ColumnValueOps::from_f64(self.sum)
        }
    }
}

pub fn get_aggregate_function<ColumnValue>(op: AggregateOp) -> Box<AggregateFunction<ColumnValue> + 'static>
where ColumnValue: Sized + ColumnValueOps + 'static
{
    match op {
        AggregateOp::Count => Box::new(Count { count: 0 }),
        AggregateOp::Avg => Box::new(Avg { sum: 0.0, count: 0 }),
        AggregateOp::Sum => Box::new(Sum { sum: 0.0, count: 0 }),
        AggregateOp::Min => unimplemented!(),
        AggregateOp::Max => unimplemented!()
    }
}
