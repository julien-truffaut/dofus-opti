use criterion::{criterion_group, criterion_main, Criterion};
use dofus_opti_dofus_build::model::{EffectsVec, EffectsArray, EffectsStruct, EffectsStructOpt};

fn bench_effects_addition(c: &mut Criterion) {
    let effects_vec1 = EffectsVec::random_sample();
    let effects_vec2 = EffectsVec::random_sample();

    c.bench_function("EffectsVec add", |b| {
        b.iter(|| {
            let mut sum = EffectsVec::empty();
            sum.add(&effects_vec1);
            sum.add(&effects_vec2);
            sum
        })
    });

    c.bench_function("EffectsVec add_ordered", |b| {
        b.iter(|| {
            let mut sum = EffectsVec::empty();
            sum.add_ordered(&effects_vec1);
            sum.add_ordered(&effects_vec2);
            sum
        })
    });

    let effects_array1 = EffectsArray::random_sample();
    let effects_array2 = EffectsArray::random_sample();

    c.bench_function("EffectArray add", |b| {
        b.iter(|| {
            let mut sum = EffectsArray::empty();
            sum.add(&effects_array1);
            sum.add(&effects_array2);
            sum
        })
    });

    let effects_struct1 = EffectsStruct::random_sample();
    let effects_struct2 = EffectsStruct::random_sample();

    c.bench_function("EffectsStruct add", |b| {
        b.iter(|| {
            let mut sum = EffectsStruct::empty();
            sum.add(&effects_struct1);
            sum.add(&effects_struct2);
            sum
        })
    });

    let effects_struct1 = EffectsStructOpt::random_sample();
    let effects_struct2 = EffectsStructOpt::random_sample();

    c.bench_function("EffectsStructOpt add", |b| {
        b.iter(|| {
            let mut sum = EffectsStructOpt::empty();
            sum.add(&effects_struct1);
            sum.add(&effects_struct2);
            sum
        })
    });
}

criterion_group!(benches, bench_effects_addition);
criterion_main!(benches);