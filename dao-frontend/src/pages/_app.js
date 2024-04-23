import { NearProvider } from '../contexts/near.context';
import { AlertProvider } from '../contexts/alert.context';
import Header from '../components/header';
import '../styles/globals.css';

function MyApp({ Component, pageProps }) {
  return (
    <AlertProvider>
      <NearProvider>
        <Header />
        <Component {...pageProps} />
      </NearProvider>
    </AlertProvider>
  );
}

export default MyApp;
