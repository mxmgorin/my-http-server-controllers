use my_http_server::HttpPath;

use super::{HttpRouteSegment, RouteValue};

pub struct HttpRoute {
    pub route: String,
    pub keys_amount: usize,
    segments: Vec<HttpRouteSegment>,
}

impl HttpRoute {
    pub fn new(route: &str) -> Self {
        let route_as_bytes = route.as_bytes();

        let mut keys_amount = 0;

        let mut segments = Vec::new();
        let mut prev_pos = None;

        let mut last_element: u8 = 0;

        for pos in 0..route_as_bytes.len() {
            last_element = route_as_bytes[pos];
            if last_element == b'/' {
                if let Some(prev_pos) = prev_pos {
                    let segment = &route_as_bytes[prev_pos + 1..pos];
                    let segment = HttpRouteSegment::new(segment);

                    if segment.is_key() {
                        keys_amount += 1;
                    }

                    segments.push(segment);
                }

                prev_pos = Some(pos);
            }
        }

        if last_element != b'/' {
            if let Some(prev_pos) = prev_pos {
                let segment = &route_as_bytes[prev_pos + 1..];
                let segment = HttpRouteSegment::new(segment);

                if segment.is_key() {
                    keys_amount += 1;
                }

                segments.push(segment);
            }
        }

        Self {
            keys_amount,
            segments,
            route: route.to_string(),
        }
    }

    pub fn is_my_path(&self, path: &HttpPath) -> bool {
        if path.segments_amount() != self.segments.len() {
            return false;
        }

        let mut index = 0;
        for segment in &self.segments {
            match segment {
                HttpRouteSegment::Key(_) => {}
                HttpRouteSegment::Segment(value) => {
                    if !path.has_value_at_index_case_insensitive(index, value) {
                        return false;
                    }
                }
            }

            index += 1;
        }

        true
    }

    pub fn get_value<'s>(&'s self, path: &'s HttpPath, key: &str) -> Option<RouteValue<'s>> {
        if self.keys_amount == 0 {
            return None;
        }

        let mut index = 0;
        for segment in &self.segments {
            match segment {
                HttpRouteSegment::Key(segment_key) => {
                    if segment_key == key {
                        let value = path.get_segment_value(index)?;
                        return Some(RouteValue::new(value));
                    }
                }
                HttpRouteSegment::Segment(_) => {}
            }

            index += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_segments() {
        let route = HttpRoute::new("/test/{key}/second");

        assert_eq!(route.segments.len(), 3);
        assert_eq!(route.segments[0].unwrap_as_segment(), "test");
        assert_eq!(route.segments[1].unwrap_as_key(), "key");
        assert_eq!(route.segments[2].unwrap_as_segment(), "second");
    }

    #[test]
    fn test_parsing_segments_with_last_slash() {
        let route = HttpRoute::new("/test/{key}/second/");

        assert_eq!(route.segments.len(), 3);
        assert_eq!(route.segments[0].unwrap_as_segment(), "test");
        assert_eq!(route.segments[1].unwrap_as_key(), "key");
        assert_eq!(route.segments[2].unwrap_as_segment(), "second");
    }

    #[test]
    fn general_test() {
        let route = HttpRoute::new("/test/{key}/second");
        let path = HttpPath::new("/test/1/second");
        assert_eq!(route.is_my_path(&path), true);

        assert_eq!(route.get_value(&path, "key").unwrap().as_str(), "1");
    }
}
