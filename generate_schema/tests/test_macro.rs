use std::collections::HashMap;

use base_types::common::{ConstraintTraits, Uid};
use base_types::traits::{GraphEnvironment, GSO};
use generate_schema::generate_concrete_schema;

#[test]
fn test_macro() {
    // let graph_environment =
    struct SampleGraphEnvironment {
        // created_instances: HashMap<Uid, Box<dyn GSO>>,
    };
    impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> GraphEnvironment<TTypes, TValues>
        for SampleGraphEnvironment
    {
    }

    let mut sge_instance = SampleGraphEnvironment {
        // created_instances: HashMap::new(),
    };

    generate_concrete_schema!(sge_instance);
}
