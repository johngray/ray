use std::ops::Index;
use std::slice::Iter;
use crate::geometry::Shape;
use crate::light::LightSource;

pub struct Scene {
    data: Vec<Box<Shape>>,
    light_sources: Vec<Box<dyn LightSource>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {data: Vec::new(), light_sources: Vec::new()}
    }

    #[inline]
    pub fn add_object(&mut self, obj: Box<Shape>) {
        self.data.push(obj)
    }

    #[inline]
    pub fn add_light_source(&mut self, light_source: Box<dyn LightSource>) {
        self.light_sources.push(light_source)
    }

    pub fn light_sources_iter(&self) -> Iter<'_, Box<dyn LightSource>> {
        self.light_sources.iter()
    }
}

impl Index<usize> for Scene {
    type Output = Box<Shape>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

pub struct SceneIter<'a> {
    offset: usize,
    collection: &'a Scene
}

impl<'a> Iterator for SceneIter<'a> {
    type Item = &'a Box<Shape>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.offset;
        self.offset += 1;
        if self.offset > self.collection.data.len() {
            return None;
        }
        Some(&self.collection[offset])
    }
}

impl<'a> IntoIterator for &'a Scene {
    type Item = &'a Box<Shape>;
    type IntoIter = SceneIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SceneIter {
            offset: 0,
            collection: self
        }
    }
}