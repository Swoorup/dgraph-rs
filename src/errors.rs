use std::error::Error;
use std::fmt;

/// The error type for Dgraph operations.
///
/// Native gRPC errors are wrapped into `GrpcError`.
#[derive(Debug)]
pub enum DgraphError {
    TxnReadOnly,
    TxnFinished,
    EmptyTxn,
    MissingTxnContext,
    WriteTxnBestEffort,
    StartTsMismatch,
    GrpcError(grpcio::Error),
}

impl Error for DgraphError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DgraphError::GrpcError(grpc_error) => Some(grpc_error),
            _ => None,
        }
    }
}

impl fmt::Display for DgraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DgraphError::TxnFinished => {
                write!(f, "Transaction has already been committed or discarded")
            }
            DgraphError::TxnReadOnly => write!(
                f,
                "Readonly transaction cannot run mutations or be committed"
            ),
            DgraphError::WriteTxnBestEffort => {
                write!(f, "Best effort only works for read-only queries")
            }
            DgraphError::EmptyTxn => write!(f, "Got empty Txn response back from query"),
            DgraphError::MissingTxnContext => write!(f, "Missing Txn context on mutation response"),
            DgraphError::StartTsMismatch => write!(f, "StartTs mismatch"),
            DgraphError::GrpcError(ref grpc_error) => write!(f, "Grpc error: {}", grpc_error),
        }
    }
}

impl From<grpcio::Error> for DgraphError {
    fn from(err: grpcio::Error) -> Self {
        DgraphError::GrpcError(err)
    }
}
