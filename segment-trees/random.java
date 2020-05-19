

class RandomN {
    
    public static void main(String args[]) {

		for (int i = 0; i < 10; ++i) {
			System.out.printf("Random for %d and %d is %d\n", i, i+1, randint(i, i+1));
		}
		System.out.printf("Random for %d and %d is %d\n", 1, 100, randint(1, 100));
    }

	public static int randint(int i, int j) {
	  return ((int)(Math.random() * (j-i+1)))+i;
	}
}

