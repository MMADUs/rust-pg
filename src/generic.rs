// generic in struct
pub struct DynamicType<T> {
  pub name: String,
  pub any: T,
}

// generic in struct impl
impl<T> DynamicType<T> {
    pub fn new(name: String, any: T) -> Self {
        DynamicType { name, any }
    }
    
    pub fn get_name(&self) -> &String {
      &self.name
    }

    pub fn get_any(&self) -> &T {
      &self.any
    }
}

// multiple generic
pub struct MultiType<T, Y, K> {
  pub t_type: T,
  pub y_type: Y,
  pub k_type: K,
}

// trait with generic
pub trait GenericTrait<T, Y, K> {
    fn new(t_type: T, y_type: Y, k_type: K) -> Self;
    fn get_t(&self) -> &T;
    fn get_y(&self) -> &Y;
    fn get_k(&self) -> &K;
    fn get_dynamic<F>(&self, value: F) -> F;
}  
  
impl<T, Y, K> GenericTrait<T, Y, K> for MultiType<T, Y, K> {
  fn new(t_type: T, y_type: Y, k_type: K) -> Self {
    MultiType { t_type, y_type, k_type }
  }

  fn get_t(&self) -> &T {
    &self.t_type
  }

  fn get_y(&self) -> &Y {
    &self.y_type
  }

  fn get_k(&self) -> &K {
    &self.k_type
  }

  fn get_dynamic<F>(&self, value: F) -> F {
    value
  }
}

// cargo test dynamic::test_dynamic
#[test]
fn test_dynamic() {
    let daniel = DynamicType::<usize>::new(String::from("Daniel"), 10);
    assert_eq!(daniel.get_name(), "Daniel");
    assert_eq!(*daniel.get_any(), 10);

    let lintang = DynamicType::<bool>::new(String::from("Lintang"), true);
    assert_eq!(lintang.get_name(), "Lintang");
    assert_eq!(*lintang.get_any(), true);

    let nizwa = DynamicType::<String>::new(String::from("Nizwa"), String::from("Ganteng"));
    assert_eq!(nizwa.get_name(), "Nizwa");
    assert_eq!(nizwa.get_any(), "Ganteng");

    let data1 = MultiType::<String, usize, bool>::new("Arfy".to_string(), 20, true);
    assert_eq!(*data1.get_t(), "Arfy".to_string());
    assert_eq!(*data1.get_y(), 20);
    assert_eq!(*data1.get_k(), true);

    let data2 = MultiType::<usize, f32, isize>::new(10, 10.829327, -10);
    assert_eq!(*data2.get_t(), 10);
    assert_eq!(*data2.get_y(), 10.829327);
    assert_eq!(*data2.get_k(), -10);
}
