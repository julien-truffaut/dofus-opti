use criterion::{Criterion, criterion_group, criterion_main};
use dofus_opti_dofus_build::model::{
    ALL_CHARACTERISTIC_TYPES, CharacteristicType, EffectsArray, EffectsStruct, EffectsStructOpt,
    EffectsVec,
};
use rand::Rng;

fn bench_effects_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("Effects add");

    let effects1 = random_effects();
    let effects2 = random_effects();

    let effects_vec1 = create_effects_vec(&effects1);
    let effects_vec2 = create_effects_vec(&effects2);

    group.bench_function("vec unordered", |b| {
        b.iter(|| {
            let mut sum = EffectsVec::empty();
            sum.add(&effects_vec1);
            sum.add(&effects_vec2);
            sum
        })
    });

    group.bench_function("vec ordered", |b| {
        b.iter(|| {
            let mut sum = EffectsVec::empty();
            sum.add_ordered(&effects_vec1);
            sum.add_ordered(&effects_vec2);
            sum
        })
    });

    let effects_array1 = create_effects_array(&effects1);
    let effects_array2 = create_effects_array(&effects2);

    group.bench_function("array", |b| {
        b.iter(|| {
            let mut sum = EffectsArray::empty();
            sum.add(&effects_array1);
            sum.add(&effects_array2);
            sum
        })
    });

    let effects_struct1 = create_effects_struct(&effects1);
    let effects_struct2 = create_effects_struct(&effects2);

    group.bench_function("struct", |b| {
        b.iter(|| {
            let mut sum = EffectsStruct::empty();
            sum.add(&effects_struct1);
            sum.add(&effects_struct2);
            sum
        })
    });

    let effects_struct_opt1 = create_effects_struct_opt(&effects1);
    let effects_struct_opt2 = create_effects_struct_opt(&effects2);

    group.bench_function("struct with option", |b| {
        b.iter(|| {
            let mut sum = EffectsStructOpt::empty();
            sum.add(&effects_struct_opt1);
            sum.add(&effects_struct_opt2);
            sum
        })
    });

    group.finish();
}

fn create_effects_vec(effects: &Vec<(CharacteristicType, i32)>) -> EffectsVec {
    let mut result = EffectsVec::empty();
    for (characteristic_type, value) in effects {
        result.set(characteristic_type, *value);
    }
    result
}

fn create_effects_array(effects: &Vec<(CharacteristicType, i32)>) -> EffectsArray {
    let mut result = EffectsArray::empty();
    for (characteristic_type, value) in effects {
        result.set(characteristic_type, *value);
    }
    result
}

fn create_effects_struct(effects: &Vec<(CharacteristicType, i32)>) -> EffectsStruct {
    let mut result = EffectsStruct::empty();
    for (characteristic_type, value) in effects {
        result.set(characteristic_type, *value);
    }
    result
}

fn create_effects_struct_opt(effects: &Vec<(CharacteristicType, i32)>) -> EffectsStructOpt {
    let mut result = EffectsStructOpt::empty();
    for (characteristic_type, value) in effects {
        result.set(characteristic_type, *value);
    }
    result
}

fn random_effects() -> Vec<(CharacteristicType, i32)> {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(5..15);

    let mut effects = vec![];

    for _ in 0..n {
        effects.push(random_effect());
    }

    effects
}

fn random_effect() -> (CharacteristicType, i32) {
    let mut rng = rand::thread_rng();
    let charac = ALL_CHARACTERISTIC_TYPES[rng.gen_range(0..ALL_CHARACTERISTIC_TYPES.len())].clone();
    let value = rng.gen_range(-50..200);

    (charac, value)
}

criterion_group!(benches, bench_effects_addition);
criterion_main!(benches);
