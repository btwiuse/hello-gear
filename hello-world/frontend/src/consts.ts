const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  DAPPS_API: process.env.REACT_APP_DAPPS_API_ADDRESS as string,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
};

const SUBHEADING = {
  LOGIN: 'In order to start, please login.',
};

export { ADDRESS, LOCAL_STORAGE, SUBHEADING };
