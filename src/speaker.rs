use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};

pub struct Speaker {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
}

impl Speaker {
    pub fn new(freq: f32, amplify: f32) -> Self {
        let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&_stream_handle).unwrap();

        let source = SineWave::new(freq).amplify(amplify);
        sink.append(source);

        Self {
            sink,
            _stream,
            _stream_handle,
        }
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.pause();
    }
}
