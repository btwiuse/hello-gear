import { Hex, Metadata } from '@gear-js/api';
import { useSendMessage } from '@gear-js/react-hooks';
import { Button, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { isHex } from '@polkadot/util';
import styles from './Home.module.scss';

const initialValues = { address: '' as Hex };
const validate = { address: (value: string) => (!isHex(value) ? 'Address should be hex' : null) };

const programId = process.env.REACT_APP_PROGRAM_ID as Hex;
const metaTypes = process.env.REACT_APP_META_TYPES as Hex;

const metadata = {
  types: metaTypes,
  init_input: 'String',
  init_output: '',
  async_init_input: '',
  async_init_output: '',
  handle_input: 'InputMessages',
  handle_output: 'String',
  async_handle_input: '',
  async_handle_output: '',
  title: 'Hello world contract',
  meta_state_input: '',
  meta_state_output: '',
} as Metadata;

function Home() {
  const form = useForm({ initialValues, validate });
  const { getInputProps, reset } = form;

  const sendMessage = useSendMessage(programId, metadata);

  const sendHello = (address: Hex) => sendMessage({ SendHelloTo: address }, { onSuccess: reset });
  const sendReply = () => sendMessage({ SendHelloReply: null });

  const handleSubmit = form.onSubmit(({ address }) => sendHello(address));

  return (
    <div className={styles.container}>
      <form onSubmit={handleSubmit} className={styles.form}>
        <Input label="Send Hello To:" direction="y" {...getInputProps('address')} block />
        <Button type="submit" text="Send Hello" />
      </form>

      <Button text="Send Reply" color="light" onClick={sendReply} />
    </div>
  );
}

export { Home };
