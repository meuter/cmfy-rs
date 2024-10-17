use crate::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub index: u64,
    pub uuid: String,
    pub nodes: PromptNodes,
    pub png_info: serde_json::Value,
    pub output_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PromptNodes(pub BTreeMap<String, Node<serde_json::Value>>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node<I> {
    pub class_type: String,
    pub inputs: I,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubmitResponse {
    pub number: u32,
    pub prompt_id: String,
    pub node_errors: serde_json::Value,
}

pub trait ClassType: Serialize {
    const CLASS_TYPE: &str;
}

impl PromptNodes {
    pub fn put<N: ClassType + Serialize>(&mut self, id: String, node: N) -> Result<()> {
        let class_type = N::CLASS_TYPE.into();
        let inputs = serde_json::to_value(&node)?;
        self.0.insert(id, Node { class_type, inputs });
        Ok(())
    }

    pub fn take<N: ClassType + DeserializeOwned>(&mut self, id: String) -> Result<N> {
        let node = self.0.remove(&id).ok_or(format!("node id '{}' not found", id))?;
        Ok(serde_json::from_value(node.inputs)?)
    }

    pub fn all_by_class<N>(&self) -> Result<BTreeMap<String, N>>
    where
        N: DeserializeOwned + ClassType,
    {
        Ok(self
            .0
            .iter()
            .filter(|(_, node)| node.class_type == N::CLASS_TYPE)
            .map(|(id, node)| {
                let serialized = serde_json::to_string(&node)?;
                let parsed: Node<N> = serde_json::from_str(&serialized)?;
                Ok::<_, serde_json::Error>((id.clone(), parsed.inputs))
            })
            .collect::<serde_json::Result<_>>()?)
    }

    pub fn first_by_class<N>(&self) -> Result<(String, N)>
    where
        N: DeserializeOwned + ClassType,
    {
        Ok(self
            .all_by_class::<N>()?
            .into_iter()
            .next()
            .ok_or(format!("node with class '{}' not found", N::CLASS_TYPE))?)
    }

    pub fn change_first_by_class<N, C>(&mut self, change: C) -> Result<()>
    where
        N: DeserializeOwned + Serialize + ClassType + std::fmt::Debug,
        C: Fn(&mut N),
    {
        let (id, mut node) = self.first_by_class()?;
        change(&mut node);
        self.put(id, node)
    }
}
