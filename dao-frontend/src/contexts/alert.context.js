import React, { createContext, useContext, useState } from 'react';
import ErrorAlert from '../components/error.alert';
import SuccessAlert from '../components/success.alert';

const AlertContext = createContext();

export const useAlert = () => useContext(AlertContext);

export const AlertProvider = ({ children }) => {
  const [alert, setAlert] = useState({ visible: false, message: '', type: '' });

  const showAlert = (message, type = 'error') => {
    setAlert({ visible: true, message, type });
  };

  const hideAlert = () => {
    setAlert({ visible: false, message: '', type: '' });
  };

  const renderAlert = () => {
    if (!alert.visible) return null;

    switch (alert.type) {
      case 'error':
        return <ErrorAlert message={alert.message} onClose={hideAlert} />;
      case 'success':
        return <SuccessAlert message={alert.message} onClose={hideAlert} />;
      default:
        return null;
    }
  };

  return (
    <AlertContext.Provider value={{ alert, showAlert, hideAlert }}>
      {children}
      {renderAlert()}
    </AlertContext.Provider>
  );
};
