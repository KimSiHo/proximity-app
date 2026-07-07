use std::collections::{HashMap, VecDeque};
use std::fmt;

use crate::protocol_bindings::DetectionPacket;

const HISTORY_SIZE: usize = 6;
const SAMPLE_INTERVAL_FRAMES: u64 = 15; // 30fps 기준 0.5초

#[derive(Clone)]
pub struct ObjectInfo {
    pub frame_num: u64,
    pub class_id: i32,
    pub label: String,
    pub confidence: f32,
    pub tracker_confidence: f32,

    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

pub struct ObjectHistory {
    pub history: VecDeque<ObjectInfo>,
}

pub struct TrackerHistory {
    objects: HashMap<u64, ObjectHistory>,
}

impl TrackerHistory {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn objects(&self) -> &HashMap<u64, ObjectHistory> {
        &self.objects
    }

    pub fn remove(&mut self, object_id: u64) {
        self.objects.remove(&object_id);
    }

    pub fn update(&mut self, packet: &DetectionPacket) {
        for i in 0..packet.object_count as usize {
            let obj = &packet.objects[i];

            let history = self
                .objects
                .entry(obj.object_id)
                .or_insert_with(|| ObjectHistory {
                    history: VecDeque::with_capacity(HISTORY_SIZE),
                });

            if let Some(last) = history.history.back()
                && packet.frame_num - last.frame_num < SAMPLE_INTERVAL_FRAMES {
                    continue;
            }

            if history.history.len() == HISTORY_SIZE {
                history.history.pop_front();
            }

            history.history.push_back(ObjectInfo {
                frame_num: packet.frame_num,
                class_id: obj.class_id,
                label: std::ffi::CStr::from_bytes_until_nul(&obj.label)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                confidence: obj.confidence,
                tracker_confidence: obj.tracker_confidence,

                left: obj.left,
                top: obj.top,
                width: obj.width,
                height: obj.height,
            });
        }
    }

    pub fn get(&self, object_id: u64) -> Option<&ObjectHistory> {
        self.objects.get(&object_id)
    }
}

impl fmt::Display for TrackerHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "objects={}", self.objects.len())?;

        for (id, history) in &self.objects {
            writeln!(f, "object_id={id}")?;
            write!(f, "{history}")?;
        }

        Ok(())
    }
}

impl fmt::Display for ObjectHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for obj in &self.history {
            writeln!(
                f,
                "frame={} class={} label={} conf={:.2} tracker_conf={:.2} ({:.1},{:.1},{:.1},{:.1})",
                obj.frame_num,
                obj.class_id,
                obj.label,
                obj.confidence,
                obj.tracker_confidence,
                obj.left,
                obj.top,
                obj.width,
                obj.height
            )?;
        }

        Ok(())
    }
}
