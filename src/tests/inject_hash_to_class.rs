#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_hash_to_class::InjectHashVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectHashVisitor::new("src/Application.njs".into(), true)),
    inject_dev_hash,
    r#"class Component extends Nullstack { works = true };"#,
    r#"class Component extends Nullstack { static hash = "src__Application___Component"; works = true };"#
);

test!(
    syntax(),
    |_| tr(InjectHashVisitor::new("src/Application.njs".into(), true)),
    inject_dev_hash_export_default,
    r#"export default class Component extends Nullstack { works = true };"#,
    r#"export default class Component extends Nullstack { static hash = "src__Application___Component"; works = true };"#
);

test!(
    syntax(),
    |_| tr(InjectHashVisitor::new(
        "src/Application/index.njs".into(),
        true
    )),
    inject_dev_hash_ignoring_index,
    r#"class Component extends Nullstack { works = true };"#,
    r#"class Component extends Nullstack { static hash = "src__Application___Component"; works = true };"#
);

test!(
    syntax(),
    |_| tr(InjectHashVisitor::new("src/Application.njs".into(), true)),
    inject_dev_hash_with_multiple_classes,
    r#"
        class Component extends Nullstack { works = true };
        class Component2 extends Nullstack { works = true };
    "#,
    r#"
        class Component extends Nullstack { static hash = "src__Application___Component"; works = true };
        class Component2 extends Nullstack { static hash = "src__Application___Component2"; works = true };
    "#
);

test!(
    Default::default(),
    |_| tr(InjectHashVisitor::new("src/Application.njs".into(), false)),
    inject_prod_hash,
    r#"class Component extends Nullstack { works = true };"#,
    r#"class Component extends Nullstack { static hash = "5b2fd539cb0f23f4"; works = true };"#
);

test!(
    Default::default(),
    |_| tr(InjectHashVisitor::new("src/Application.njs".into(), false)),
    inject_prod_hash_export_default,
    r#"export default class Component extends Nullstack { works = true };"#,
    r#"export default class Component extends Nullstack { static hash = "5b2fd539cb0f23f4"; works = true };"#
);

test!(
    Default::default(),
    |_| tr(InjectHashVisitor::new(
        "src/Application/index.njs".into(),
        false
    )),
    inject_prod_hash_ignoring_index,
    r#"class Component extends Nullstack { works = true };"#,
    r#"class Component extends Nullstack { static hash = "5b2fd539cb0f23f4"; works = true };"#
);

test!(
    Default::default(),
    |_| tr(InjectHashVisitor::new("src/Application.njs".into(), false)),
    inject_prod_hash_with_multiple_classes,
    r#"
        class Component extends Nullstack { works = true };
        class Component2 extends Nullstack { works = true };
    "#,
    r#"
        class Component extends Nullstack { static hash = "5b2fd539cb0f23f4"; works = true };
        class Component2 extends Nullstack { static hash = "5b2fd539a0ce872b"; works = true };
    "#
);
