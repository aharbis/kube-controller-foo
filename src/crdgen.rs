use kube::CustomResourceExt;
use kube_controller_foo::foo::Foo;

fn main() {
    print!("{}", serde_yaml::to_string(&Foo::crd()).unwrap())
}
