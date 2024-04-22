import { NearProvider } from '../contexts/near.context';
import Header from '../components/header';
import '../styles/globals.css';

function MyApp({ Component, pageProps }) {
  return (
    <NearProvider>
        <Header />
        <Component {...pageProps} />
    </NearProvider>
  );
}

export default MyApp;
