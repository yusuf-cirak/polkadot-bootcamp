// The 'filter' field is a function that takes a reference to an item and returns a boolean value
pub struct FilterCondition<T> {
    filter: fn(item: &T) -> bool,
}

impl<T> FilterCondition<T> {
    pub fn new(filter: fn(item: &T) -> bool) -> Self {
        FilterCondition { filter }
    }
    // The 'is_match' method takes a reference to an item and returns a boolean value
    // We're making sure that is_match method can only be used with a FiterCondition instance by using &self. Also we can use the dot operator to access to method.
    // We must use &self instead of self because we don't want to take ownership of the instance. We just want to borrow it.
    // We need to get the reference of item because its in a collection so went take the items reference.
    pub fn is_match(&self, item: &T) -> bool {
        (self.filter)(item)
    }
}

// We cant take the ownership of the collection because we need to use it after the function call. Thats why we're using a reference.
pub fn custom_filter<T>(collection: &[T], filter_condition: &FilterCondition<T>) -> Vec<T>
where
    T: std::clone::Clone, // We're filtering with a reference to the item and also we need to return a new vector instance. This is why T must implement the Clone trait.
{
    let filtered_collection: Vec<T> = collection
        .iter()
        .filter(|&item| filter_condition.is_match(item)) // Execute the is_match method with the item reference
        .cloned()
        .collect();

    filtered_collection
}
