use crate::*;

impl ResponseTrait for TcpResponseText {
    type OutputText = TcpResponseText;
    type OutputBinary = TcpResponseBinary;

    #[inline]
    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <TcpResponseBinary as ResponseTrait>::from(response).text()
    }

    #[inline]
    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    #[inline]
    fn binary(&self) -> TcpResponseBinary {
        self.clone().into_bytes()
    }
}
