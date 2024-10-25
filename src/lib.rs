use bevy::prelude::*;

#[derive(Component)]
pub struct TreeNode;

pub enum NodeId {
    Node(String),
    Button(String),
    Image(String),
    Text(String),
}

pub struct Builder {
}

impl Builder {
    pub fn new_node(&mut self, id: String, bundle: NodeBundle) -> NodeBuilder<'_> {
        NodeBuilder {
            builder: self,
        }
    }

    pub fn new_button(&mut self, id: String, bundle: ButtonBundle) -> NodeBuilder<'_> {
        NodeBuilder {
            builder: self,
        }
    }

    pub fn new_image(&mut self, id: String, bundle: ImageBundle) -> NodeBuilder<'_> {
        NodeBuilder {
            builder: self,
        }
    }

    pub fn new_text(&mut self, id: String, bundle: TextBundle) -> NodeBuilder<'_> {
        NodeBuilder {
            builder: self,
        }
    }
}

pub struct NodeBuilder<'a> {
    builder: &'a mut Builder,
}

impl NodeBuilder<'_> {
    pub fn with_children(&self, f: impl FnOnce(&mut Builder)) {

    }
}

pub struct State {

}

pub fn render(builder: &Builder, state: &State) {

}