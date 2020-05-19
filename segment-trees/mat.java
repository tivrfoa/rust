import java.util.*;

class C {
    
    public static void main(String args[]) {
		int	[][] v1 = new int[][]{
               {1, 0, 3, 2},
               {1, 0, 7, 1},
               {1, 4, 7, 10},
               {1, 2, 5, 7},
               {2, 0, 7},
               {2, 1, 4},
               {2, 2, 5},
               {2, 0, 0},
               {2, 1, 1},
               {2, 2, 2},
               {2, 3, 3},
               {2, 4, 4},
               {2, 5, 5},
         };

		System.out.println(v1.length);
		 for (var a1 : v1) {
			 System.out.print(a1.length + ": ");
			 for (var i : a1)
			 	System.out.print(i + " ");
			System.out.println();
		 }
    }
}

