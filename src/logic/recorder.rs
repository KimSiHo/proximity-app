use crate::logic::command::Command;
use crate::tracking::history::TrackerHistory;

const PERSON_CLASS_ID: i32 = 2;
const MIN_HISTORY: usize = 5;
const AREA_THRESHOLD: f32 = 3.0;

pub struct Recorder;

impl Recorder {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, tracker: &mut TrackerHistory) -> Option<Command> {
        for (object_id, history) in tracker.objects() {
            let frames = &history.history;

            if frames.len() < MIN_HISTORY {
                continue;
            }

            let first = &frames.front().unwrap();
            let last = &frames.back().unwrap();

            if last.class_id != PERSON_CLASS_ID {
                continue;
            }

            let first_area = first.width * first.height;
            let last_area = last.width * last.height;

            if last_area > first_area * AREA_THRESHOLD {
                println!("object_id={object_id} first_area={first_area:.1} last_area={last_area:.1}");

                let id = *object_id;
                tracker.remove(id);

                return Some(Command::StartRecording);
            }
        }

        None
    }
}
